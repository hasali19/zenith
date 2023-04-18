use camino::Utf8Path;
use db::media::{MediaItemType, MetadataProvider};
use db::sql;
use time::OffsetDateTime;

use crate::library::parser::MoviePathMeta;

use super::{LibraryEvent, MediaLibrary};

impl MediaLibrary {
    pub async fn import_movie(&self, path: &Utf8Path) -> eyre::Result<()> {
        let Some(MoviePathMeta { name, year }) = self.parser().parse_movie_path(path) else {
            return Ok(());
        };

        tracing::info!(%path, "importing movie");

        let movie_id = self.create_movie(path, &name, year).await?;

        let _ = self
            .notifier
            .send(LibraryEvent::MediaAdded(MediaItemType::Movie, movie_id));

        self.create_video_file(path, movie_id).await
    }

    async fn create_movie(
        &self,
        path: &Utf8Path,
        name: &str,
        year: Option<OffsetDateTime>,
    ) -> eyre::Result<i64> {
        tracing::info!("adding movie: {name}");

        let mut transaction = self.db.begin().await?;

        let sql = sql::insert("media_items")
            .columns(&["item_type", "name", "start_date", "metadata_provider"])
            .values(&["?", "?", "?", "?"])
            .returning(&["id"])
            .to_sql();

        let movie_id: i64 = sqlx::query_scalar(&sql)
            .bind(MediaItemType::Movie)
            .bind(name)
            .bind(year.map(|dt| dt.unix_timestamp()))
            .bind(MetadataProvider::Tmdb)
            .fetch_one(&mut transaction)
            .await?;

        let sql = sql::insert("indexed_paths")
            .columns(&["item_id", "path"])
            .values(&["?", "?"])
            .to_sql();

        sqlx::query(&sql)
            .bind(movie_id)
            .bind(path.parent().unwrap())
            .execute(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(movie_id)
    }

    pub(super) async fn validate_movies(&self) -> eyre::Result<()> {
        tracing::info!("validating movies");

        let mut conn = self.db.acquire().await?;

        let sql = sql::select("media_items")
            .columns(&["id", "name"])
            .condition("item_type = ? AND id NOT IN (SELECT item_id FROM video_files)")
            .to_sql();

        let ids: Vec<(i64, String)> = sqlx::query_as(&sql)
            .bind(MediaItemType::Movie)
            .fetch_all(&mut conn)
            .await?;

        for (id, name) in ids {
            tracing::info!(name, "removing movie");
            self.remove_item(&mut conn, id, MediaItemType::Movie)
                .await?;
        }

        Ok(())
    }
}
