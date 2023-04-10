use camino::Utf8Path;

use crate::db;
use crate::db::media::{MediaItemType, MetadataProvider};

use super::{video_info, LibraryEvent, MediaLibrary};

pub struct NewMovie<'a> {
    pub parent_path: &'a Utf8Path,
    pub path: &'a Utf8Path,
    pub title: &'a str,
    pub release_date: Option<i64>,
}

impl MediaLibrary {
    /// Adds a new movie
    pub async fn add_movie(&self, movie: &NewMovie<'_>) -> eyre::Result<i64> {
        let info = self.video_prober.probe(movie.path).await?;
        let duration: f64 = info.format.duration.parse()?;
        let mut transaction = self.db.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type, name, start_date, metadata_provider)
            VALUES (?, ?, ?, ?)
        ";

        let id = sqlx::query(sql)
            .bind(MediaItemType::Movie)
            .bind(movie.title)
            .bind(movie.release_date)
            .bind(MetadataProvider::Tmdb)
            .execute(&mut transaction)
            .await?
            .last_insert_rowid();

        let sql = "
            INSERT INTO indexed_paths (item_id, path)
            VALUES (?, ?)";

        sqlx::query(sql)
            .bind(id)
            .bind(movie.parent_path)
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

        video_info::update_video_info(&mut transaction, id, &info).await?;

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::Added(MediaItemType::Movie, id));

        Ok(id)
    }

    /// Removes a single movie by id
    pub async fn remove_movie(&self, id: i64) -> eyre::Result<()> {
        let mut transaction = self.db.begin().await?;
        db::items::remove(&mut transaction, id).await?;
        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::Removed(MediaItemType::Movie, id));

        Ok(())
    }

    /// Checks if a movie exists with the given path
    pub async fn get_movie_id_by_path(&self, path: &Utf8Path) -> eyre::Result<Option<i64>> {
        let sql = "
            SELECT m.id FROM movies AS m
            JOIN video_files AS v ON m.id = v.item_id
            WHERE v.path = ?
        ";

        let id = sqlx::query_scalar(sql)
            .bind(path)
            .fetch_optional(&mut *self.db.acquire().await?)
            .await?;

        Ok(id)
    }

    /// Removes any movies that no longer exist on the filesystem
    pub async fn validate_movies(&self) -> eyre::Result<()> {
        tracing::info!("validating movies");

        let mut conn = self.db.acquire().await?;

        let sql = "
            SELECT movies.id, path FROM movies
            JOIN video_files ON movies.id = video_files.item_id
        ";

        let movies: Vec<(i64, String)> = sqlx::query_as(sql).fetch_all(&mut conn).await?;

        for (id, path) in movies {
            // Check if file exists
            if !Utf8Path::new(&path).is_file() {
                tracing::info!("{path} does not exist, removing movie");
                self.remove_movie(id).await?;
            }
        }

        Ok(())
    }
}
