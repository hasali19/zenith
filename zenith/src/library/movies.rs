use std::path::Path;
use std::sync::Arc;

use crate::db::media::MediaItemType;
use crate::db::Db;
use crate::ffprobe::VideoInfoProvider;

pub struct MovieLibrary {
    db: Db,
    video_info: Arc<dyn VideoInfoProvider>,
}

pub struct NewMovie<'a> {
    pub path: &'a str,
    pub title: &'a str,
    pub release_date: Option<i64>,
}

impl MovieLibrary {
    pub fn new(db: Db, video_info: Arc<dyn VideoInfoProvider>) -> MovieLibrary {
        MovieLibrary { db, video_info }
    }

    /// Adds a new movie
    pub async fn add_movie(&self, movie: &NewMovie<'_>) -> eyre::Result<i64> {
        let info = self.video_info.get_video_info(movie.path).await?;
        let duration: f64 = info.format.duration.parse()?;
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
            INSERT INTO movies (item_id, title, release_date)
            VALUES (?, ?, ?)
        ";

        sqlx::query(sql)
            .bind(id)
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
            .bind(duration)
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(id)
    }

    /// Removes a single movie by id
    pub async fn remove_movie(&self, id: i64) -> eyre::Result<()> {
        let mut transaction = self.db.begin().await?;

        let sql = "
            SELECT path FROM movies AS m
            JOIN video_files AS v ON m.item_id = v.item_id
            WHERE m.item_id = ?
        ";

        let path: Option<String> = sqlx::query_scalar(sql)
            .bind(id)
            .fetch_optional(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM user_item_data WHERE item_id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM video_files WHERE item_id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM movies WHERE item_id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        sqlx::query("DELETE FROM media_items WHERE id = ?")
            .bind(id)
            .execute(&mut transaction)
            .await?;

        if let Some(path) = path {
            if Path::new(&path).is_file() {
                std::fs::remove_file(&path)?;
            }
        }

        transaction.commit().await?;

        Ok(())
    }

    /// Checks if a movie exists with the given path
    pub async fn exists_by_path(&self, path: &str) -> eyre::Result<bool> {
        let sql = "
            SELECT m.item_id FROM movies AS m
            JOIN video_files AS v ON m.item_id = v.item_id
            WHERE v.path = ?
        ";

        let id = sqlx::query(sql)
            .bind(path)
            .fetch_optional(&mut *self.db.acquire().await?)
            .await?;

        Ok(id.is_some())
    }

    /// Removes any movies that no longer exist on the filesystem
    pub async fn validate(&self) -> eyre::Result<()> {
        let mut conn = self.db.acquire().await?;

        let sql = "
            SELECT item_id, path FROM movies
            JOIN video_files USING (item_id)
        ";

        let movies: Vec<(i64, String)> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

        for (id, path) in movies {
            // Check if file exists
            if !Path::new(&path).is_file() {
                tracing::info!("{} does not exist, removing movie", path);
                self.remove_movie(id).await?;
            }
        }

        Ok(())
    }
}
