use sqlx::Connection;

use crate::db::media::MediaItemType;
use crate::db::Db;
use crate::metadata::{MetadataManager, RefreshRequest};

pub struct MediaLibraryImpl {
    db: Db,
    metadata: MetadataManager,
}

impl MediaLibraryImpl {
    pub fn new(db: Db, metadata: MetadataManager) -> Self {
        MediaLibraryImpl { db, metadata }
    }
}

pub struct NewMovie<'a> {
    pub path: &'a str,
    pub title: &'a str,
    pub release_date: Option<i64>,
    pub duration: f64,
}

pub struct NewShow<'a> {
    pub path: &'a str,
    pub name: &'a str,
}

pub struct NewSeason {
    pub show_id: i64,
    pub season_number: i32,
}

pub struct NewEpisode<'a> {
    pub season_id: i64,
    pub episode_number: i32,
    pub path: &'a str,
    pub duration: f64,
}

#[async_trait::async_trait]
pub trait MediaLibrary {
    async fn get_movie_ids(&self) -> eyre::Result<Vec<i64>>;
    async fn get_movie_id(&self, path: &str) -> eyre::Result<Option<i64>>;
    async fn add_movie<'a>(&self, movie: NewMovie<'a>) -> eyre::Result<()>;
    async fn remove_movie(&self, id: i64) -> eyre::Result<()>;

    async fn get_show_ids(&self) -> eyre::Result<Vec<i64>>;
    async fn get_show_id(&self, path: &str) -> eyre::Result<Option<i64>>;
    async fn add_show<'a>(&self, show: NewShow<'a>) -> eyre::Result<i64>;
    async fn remove_show(&self, id: i64) -> eyre::Result<()>;

    async fn get_season_ids(&self, show_id: i64) -> eyre::Result<Vec<i64>>;
    async fn get_season_id(&self, show_id: i64, number: i32) -> eyre::Result<Option<i64>>;
    async fn add_season(&self, season: NewSeason) -> eyre::Result<i64>;
    async fn remove_season(&self, id: i64) -> eyre::Result<()>;

    async fn get_episode_ids(&self, season_id: i64) -> eyre::Result<Vec<i64>>;
    async fn get_episode_id(&self, season_id: i64, number: i32) -> eyre::Result<Option<i64>>;
    async fn add_episode<'a>(&self, episode: NewEpisode<'a>) -> eyre::Result<i64>;
    async fn remove_episode(&self, id: i64) -> eyre::Result<()>;
}

#[async_trait::async_trait]
impl MediaLibrary for MediaLibraryImpl {
    async fn get_movie_ids(&self) -> eyre::Result<Vec<i64>> {
        Ok(
            sqlx::query_scalar("SELECT id FROM media_items WHERE item_type = 1")
                .fetch_all(&mut *self.db.acquire().await?)
                .await?,
        )
    }

    async fn get_movie_id(&self, path: &str) -> eyre::Result<Option<i64>> {
        Ok(
            sqlx::query_scalar("SELECT item_id FROM movies WHERE path = ?")
                .bind(path)
                .fetch_optional(&mut *self.db.acquire().await?)
                .await?,
        )
    }

    async fn add_movie<'a>(&self, movie: NewMovie<'a>) -> eyre::Result<()> {
        let mut transaction = self.db.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type)
            VALUES (?)
        ";

        let id: i64 = sqlx::query(sql)
            .bind(MediaItemType::Movie)
            .execute(&mut transaction)
            .await?
            .last_insert_rowid();

        let sql = "
            INSERT INTO movies (item_id, path, title, release_date)
            VALUES (?, ?, ?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
            .bind(movie.path)
            .bind(movie.title)
            .bind(movie.release_date)
            .execute(&mut transaction)
            .await?;

        let sql = "
            INSERT INTO video_files (item_id, path, duration)
            VALUES (?, ?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
            .bind(movie.path)
            .bind(movie.duration)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        self.metadata.enqueue(RefreshRequest::Movie(id));

        Ok(())
    }

    async fn remove_movie(&self, id: i64) -> eyre::Result<()> {
        sqlx::query("DELETE FROM media_items WHERE id = ?")
            .bind(id)
            .execute(&mut *self.db.acquire().await?)
            .await?;

        Ok(())
    }

    async fn get_show_ids(&self) -> eyre::Result<Vec<i64>> {
        Ok(sqlx::query_scalar("SELECT item_id FROM tv_shows")
            .fetch_all(&mut *self.db.acquire().await?)
            .await?)
    }

    async fn get_show_id(&self, path: &str) -> eyre::Result<Option<i64>> {
        Ok(
            sqlx::query_scalar("SELECT item_id FROM tv_shows WHERE path = ?")
                .bind(path)
                .fetch_optional(&mut *self.db.acquire().await?)
                .await?,
        )
    }

    async fn add_show<'a>(&self, show: NewShow<'a>) -> eyre::Result<i64> {
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

        self.metadata.enqueue(RefreshRequest::TvShow(id));

        Ok(id)
    }

    async fn remove_show(&self, id: i64) -> eyre::Result<()> {
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

    async fn get_season_ids(&self, show_id: i64) -> eyre::Result<Vec<i64>> {
        Ok(
            sqlx::query_scalar("SELECT item_id FROM tv_seasons WHERE show_id = ?")
                .bind(show_id)
                .fetch_all(&mut *self.db.acquire().await?)
                .await?,
        )
    }

    async fn get_season_id(&self, show_id: i64, number: i32) -> eyre::Result<Option<i64>> {
        let sql = "
            SELECT item_id FROM tv_seasons
            WHERE show_id = ? AND season_number = ?
        ";

        Ok(sqlx::query_scalar(sql)
            .bind(show_id)
            .bind(number)
            .fetch_optional(&mut *self.db.acquire().await?)
            .await?)
    }

    async fn add_season(&self, season: NewSeason) -> eyre::Result<i64> {
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

        self.metadata.enqueue(RefreshRequest::TvSeason(id));

        Ok(id)
    }

    async fn remove_season(&self, id: i64) -> eyre::Result<()> {
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

    async fn get_episode_ids(&self, season_id: i64) -> eyre::Result<Vec<i64>> {
        Ok(
            sqlx::query_scalar("SELECT item_id FROM tv_episodes WHERE season_id = ?")
                .bind(season_id)
                .fetch_all(&mut *self.db.acquire().await?)
                .await?,
        )
    }

    async fn get_episode_id(&self, season_id: i64, number: i32) -> eyre::Result<Option<i64>> {
        let sql = "
            SELECT item_id FROM tv_episodes
            WHERE season_id = ? AND episode_number = ?
        ";

        let res = sqlx::query_scalar(sql)
            .bind(season_id)
            .bind(number)
            .fetch_optional(&mut *self.db.acquire().await?)
            .await?;

        Ok(res)
    }

    async fn add_episode<'a>(&self, episode: NewEpisode<'a>) -> eyre::Result<i64> {
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
            INSERT INTO tv_episodes (item_id, season_id, episode_number)
            VALUES (?, ?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
            .bind(episode.season_id)
            .bind(episode.episode_number)
            .execute(&mut *transaction)
            .await?;

        let sql = "
            INSERT INTO video_files (item_id, path, duration)
            VALUES (?, ?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
            .bind(episode.path)
            .bind(episode.duration)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        self.metadata.enqueue(RefreshRequest::TvEpisode(id));

        Ok(id)
    }

    async fn remove_episode(&self, id: i64) -> eyre::Result<()> {
        let mut transaction = self.db.begin().await?;

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

        transaction.commit().await?;

        Ok(())
    }
}
