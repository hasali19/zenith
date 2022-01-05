use std::path::Path;
use std::sync::Arc;

use sqlx::Connection;

use crate::db::media::MediaItemType;
use crate::db::Db;
use crate::ffprobe::VideoInfoProvider;

use super::video_info;

pub struct ShowLibrary {
    db: Db,
    video_info: Arc<dyn VideoInfoProvider>,
}

pub struct NewShow<'a> {
    pub path: &'a str,
    pub name: &'a str,
}

pub struct NewSeason {
    pub show_id: i64,
    pub season_number: u32,
}

pub struct NewEpisode<'a> {
    pub season_id: i64,
    pub episode_number: u32,
    pub name: Option<&'a str>,
    pub path: &'a str,
}

impl ShowLibrary {
    pub fn new(db: Db, video_info: Arc<dyn VideoInfoProvider>) -> ShowLibrary {
        ShowLibrary { db, video_info }
    }

    /// Adds a new show
    pub async fn add_show(&self, show: NewShow<'_>) -> eyre::Result<i64> {
        let mut transaction = self.db.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type)
            VALUES (?)
        ";

        let id: i64 = sqlx::query(sql)
            .bind(MediaItemType::TvShow)
            .execute(&mut transaction)
            .await?
            .last_insert_rowid();

        let sql = "
            INSERT INTO tv_shows (item_id, path, name)
            VALUES (?, ?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
            .bind(show.path)
            .bind(show.name)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(id)
    }

    /// Removes a show by id
    ///
    /// This will also remove any associated seasons and episodes
    pub async fn remove_show(&self, id: i64) -> eyre::Result<()> {
        let mut db = self.db.acquire().await?;

        let seasons: Vec<i64> =
            sqlx::query_scalar("SELECT item_id FROM tv_seasons WHERE show_id = ?")
                .bind(id)
                .fetch_all(&mut *db)
                .await?;

        for season in seasons {
            self.remove_season(season).await?;
        }

        let mut transaction = db.begin().await?;

        sqlx::query("DELETE FROM tv_shows WHERE item_id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM media_items WHERE id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    /// Retrieves a show id by path
    pub async fn get_show_id_by_path(&self, path: &str) -> eyre::Result<Option<i64>> {
        let id = sqlx::query_scalar("SELECT item_id FROM tv_shows WHERE path = ?")
            .bind(path)
            .fetch_optional(&mut *self.db.acquire().await?)
            .await?;

        Ok(id)
    }

    /// Adds a new season
    pub async fn add_season(&self, season: NewSeason) -> eyre::Result<i64> {
        let mut transaction = self.db.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type)
            VALUES (?)
        ";

        let id: i64 = sqlx::query(sql)
            .bind(MediaItemType::TvSeason)
            .execute(&mut *transaction)
            .await?
            .last_insert_rowid();

        let sql = "
            INSERT INTO tv_seasons (item_id, show_id, season_number)
            VALUES (?, ?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
            .bind(season.show_id)
            .bind(season.season_number)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(id)
    }

    /// Removes a season
    ///
    /// This will also remove any associated episodes
    pub async fn remove_season(&self, id: i64) -> eyre::Result<()> {
        let mut db = self.db.acquire().await?;

        let episodes: Vec<i64> =
            sqlx::query_scalar("SELECT item_id FROM tv_episodes WHERE season_id = ?")
                .bind(id)
                .fetch_all(&mut *db)
                .await?;

        for episode in episodes {
            self.remove_episode(episode).await?;
        }

        let mut transaction = db.begin().await?;

        sqlx::query("DELETE FROM tv_seasons WHERE item_id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM media_items WHERE id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    /// Retrieves a season id from a show id and season number
    pub async fn get_season_id(
        &self,
        show_id: i64,
        season_number: u32,
    ) -> eyre::Result<Option<i64>> {
        let sql = "
            SELECT item_id FROM tv_seasons
            WHERE show_id = ? AND season_number = ?
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
        let info = self.video_info.get_video_info(episode.path).await?;
        let duration: f64 = info.format.duration.parse()?;
        let mut transaction = self.db.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type)
            VALUES (?)
        ";

        let id: i64 = sqlx::query(sql)
            .bind(MediaItemType::TvEpisode)
            .execute(&mut *transaction)
            .await?
            .last_insert_rowid();

        let sql = "
            INSERT INTO tv_episodes (item_id, season_id, episode_number, name)
            VALUES (?, ?, ?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
            .bind(episode.season_id)
            .bind(episode.episode_number)
            .bind(episode.name)
            .execute(&mut *transaction)
            .await?;

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

        sqlx::query("DELETE FROM tv_episodes WHERE item_id = ?")
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

        Ok(())
    }

    /// Retrieves an episode id from a season id and episode number
    pub async fn get_episode_id(
        &self,
        season_id: i64,
        episode_number: u32,
    ) -> eyre::Result<Option<i64>> {
        let sql = "
            SELECT item_id FROM tv_episodes
            WHERE season_id = ? AND episode_number = ?
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
    pub async fn validate(&self) -> eyre::Result<()> {
        let mut conn = self.db.acquire().await?;

        let sql = "
            SELECT item_id, path FROM tv_episodes
            JOIN video_files USING (item_id)
        ";

        let episodes: Vec<(i64, String)> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

        for (id, path) in episodes {
            // Check if file exists
            if !Path::new(&path).is_file() {
                tracing::info!("{} does not exist, removing episode (id: {})", path, id);
                self.remove_episode(id).await?;
            }
        }

        let sql = "
            SELECT item_id AS season FROM tv_seasons
            WHERE NOT EXISTS (
                SELECT item_id FROM tv_episodes
                WHERE season_id = season
            )
        ";

        let seasons: Vec<i64> = sqlx::query_scalar(sql).fetch_all(&mut conn).await?;

        for id in seasons {
            tracing::info!("season (id: {}) has no episodes, removing", id);
            self.remove_season(id).await?;
        }

        let sql = "
            SELECT item_id AS show FROM tv_shows
            WHERE NOT EXISTS (
                SELECT item_id FROM tv_seasons
                WHERE show_id = show
            )
        ";

        let shows: Vec<i64> = sqlx::query_scalar(sql).fetch_all(&mut conn).await?;

        for id in shows {
            tracing::info!("show (id: {}) has no seasons, removing", id);
            self.remove_show(id).await?;
        }

        Ok(())
    }
}
