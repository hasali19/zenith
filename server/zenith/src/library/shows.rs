use std::path::Path;

use sqlx::Connection;

use crate::db::media::MediaItemType;

use super::{video_info, LibraryEvent, MediaLibrary};

pub struct NewShow<'a> {
    pub path: &'a str,
    pub name: &'a str,
}

pub struct NewSeason {
    pub show_id: i64,
    pub season_number: u32,
}

pub struct NewEpisode<'a> {
    pub show_id: i64,
    pub season_id: i64,
    pub season_number: u32,
    pub episode_number: u32,
    pub name: &'a str,
    pub path: &'a str,
}

impl MediaLibrary {
    /// Adds a new show
    pub async fn add_show(&self, show: NewShow<'_>) -> eyre::Result<i64> {
        let mut transaction = self.db.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type, name)
            VALUES (?, ?)
        ";

        let id: i64 = sqlx::query(sql)
            .bind(MediaItemType::Show)
            .bind(show.name)
            .execute(&mut transaction)
            .await?
            .last_insert_rowid();

        let sql = "
            INSERT INTO indexed_paths (item_id, path)
            VALUES (?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
            .bind(show.path)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::Added(MediaItemType::Show, id));

        Ok(id)
    }

    /// Removes a show by id
    ///
    /// This will also remove any associated seasons and episodes
    pub async fn remove_show(&self, id: i64) -> eyre::Result<()> {
        let mut db = self.db.acquire().await?;

        let seasons: Vec<i64> = sqlx::query_scalar("SELECT id FROM seasons WHERE show_id = ?")
            .bind(id)
            .fetch_all(&mut *db)
            .await?;

        for season in seasons {
            self.remove_season(season).await?;
        }

        let mut transaction = db.begin().await?;

        sqlx::query("DELETE FROM media_items WHERE id = ? AND item_type = ?")
            .bind(id)
            .bind(MediaItemType::Show)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::Removed(MediaItemType::Show, id));

        Ok(())
    }

    /// Retrieves a show id by path
    pub async fn get_show_id_by_path(&self, path: &str) -> eyre::Result<Option<i64>> {
        let sql = "
            SELECT item_id FROM indexed_paths
            JOIN shows ON shows.id = item_id
            WHERE path = ?";

        let id = sqlx::query_scalar(sql)
            .bind(path)
            .fetch_optional(&mut *self.db.acquire().await?)
            .await?;

        Ok(id)
    }

    /// Adds a new season
    pub async fn add_season(&self, season: NewSeason) -> eyre::Result<i64> {
        let mut transaction = self.db.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type, name, parent_id, parent_index)
            VALUES (?, ?, ?, ?)
        ";

        let id: i64 = sqlx::query(sql)
            .bind(MediaItemType::Season)
            .bind(format!("Season {}", season.season_number))
            .bind(season.show_id)
            .bind(season.season_number)
            .execute(&mut *transaction)
            .await?
            .last_insert_rowid();

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::Added(MediaItemType::Season, id));

        Ok(id)
    }

    /// Removes a season
    ///
    /// This will also remove any associated episodes
    pub async fn remove_season(&self, id: i64) -> eyre::Result<()> {
        let mut db = self.db.acquire().await?;

        let episodes: Vec<i64> = sqlx::query_scalar("SELECT id FROM episodes WHERE season_id = ?")
            .bind(id)
            .fetch_all(&mut *db)
            .await?;

        for episode in episodes {
            self.remove_episode(episode).await?;
        }

        let mut transaction = db.begin().await?;

        sqlx::query("DELETE FROM media_items WHERE id = ? AND item_type = ?")
            .bind(id)
            .bind(MediaItemType::Season)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::Removed(MediaItemType::Season, id));

        Ok(())
    }

    /// Retrieves a season id from a show id and season number
    pub async fn get_season_id(
        &self,
        show_id: i64,
        season_number: u32,
    ) -> eyre::Result<Option<i64>> {
        let sql = "
            SELECT id FROM seasons
            WHERE show_id = ? AND season_no = ?
        ";

        let id = sqlx::query_scalar(sql)
            .bind(show_id)
            .bind(season_number)
            .fetch_optional(&mut *self.db.acquire().await?)
            .await?;

        Ok(id)
    }

    /// Adds a new episode
    pub async fn add_episode(&self, episode: NewEpisode<'_>) -> eyre::Result<i64> {
        let info = self.video_prober.probe(episode.path).await?;
        let duration: f64 = info.format.duration.parse()?;
        let mut transaction = self.db.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type, name, parent_id, parent_index, grandparent_id, grandparent_index)
            VALUES (?, ?, ?, ?, ?, ?)
        ";

        let id = sqlx::query(sql)
            .bind(MediaItemType::Episode)
            .bind(episode.name)
            .bind(episode.season_id)
            .bind(episode.episode_number)
            .bind(episode.show_id)
            .bind(episode.season_number)
            .execute(&mut *transaction)
            .await?
            .last_insert_rowid();

        let sql = "
            INSERT INTO video_files (item_id, path, duration)
            VALUES (?, ?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
            .bind(episode.path)
            .bind(duration)
            .execute(&mut *transaction)
            .await?;

        video_info::update_video_info(&mut *transaction, id, &info).await?;

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::Added(MediaItemType::Episode, id));

        Ok(id)
    }

    /// Removes an episode
    ///
    /// This will also delete the episode file from the filesystem, if it exists
    pub async fn remove_episode(&self, id: i64) -> eyre::Result<()> {
        let mut transaction = self.db.begin().await?;

        let path: String = sqlx::query_scalar("SELECT path FROM video_files WHERE item_id = ?")
            .bind(id)
            .fetch_one(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM user_item_data WHERE item_id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM subtitles WHERE video_id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM video_file_streams WHERE video_id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM video_files WHERE item_id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM media_items WHERE id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        if Path::new(&path).is_file() {
            std::fs::remove_file(&path)?;
        }

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::Removed(MediaItemType::Episode, id));

        Ok(())
    }

    /// Retrieves an episode id from a season id and episode number
    pub async fn get_episode_id(
        &self,
        season_id: i64,
        episode_number: u32,
    ) -> eyre::Result<Option<i64>> {
        let sql = "
            SELECT id FROM episodes
            WHERE season_id = ? AND episode_no = ?
        ";

        let id = sqlx::query_scalar(sql)
            .bind(season_id)
            .bind(episode_number)
            .fetch_optional(&mut *self.db.acquire().await?)
            .await?;

        Ok(id)
    }

    /// Validates the tv shows stored in the database
    ///
    /// This will delete any episodes that don't exist anymore, seasons
    /// that don't have any episodes and shows that don't have any seasons.
    pub async fn validate_shows(&self) -> eyre::Result<()> {
        let mut conn = self.db.acquire().await?;

        let sql = "
            SELECT episodes.id, path FROM episodes
            JOIN video_files ON episodes.id = video_files.item_id
        ";

        let episodes: Vec<(i64, String)> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

        for (id, path) in episodes {
            // Check if file exists
            if !Path::new(&path).is_file() {
                tracing::info!("{path} does not exist, removing episode (id: {id})");
                self.remove_episode(id).await?;
            }
        }

        drop(conn);

        self.remove_empty_collections().await
    }

    pub async fn remove_empty_collections(&self) -> eyre::Result<()> {
        let mut conn = self.db.acquire().await?;

        let sql = "
            SELECT id AS season FROM seasons
            WHERE NOT EXISTS (
                SELECT id FROM episodes
                WHERE season_id = season
            )
        ";

        let seasons: Vec<i64> = sqlx::query_scalar(sql).fetch_all(&mut conn).await?;

        for id in seasons {
            tracing::info!("season (id: {id}) has no episodes, removing");
            self.remove_season(id).await?;
        }

        let sql = "
            SELECT id AS show FROM shows
            WHERE NOT EXISTS (
                SELECT id FROM seasons
                WHERE show_id = show
            )
        ";

        let shows: Vec<i64> = sqlx::query_scalar(sql).fetch_all(&mut conn).await?;

        for id in shows {
            tracing::info!("show (id: {id}) has no seasons, removing");
            self.remove_show(id).await?;
        }

        Ok(())
    }
}
