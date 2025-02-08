use std::borrow::Cow;

use camino::Utf8Path;
use db::media::{MediaItemType, MetadataProvider};
use db::sql::{self, Join};
use db::{ReadConnection, WriteConnection};
use eyre::Context;

use super::parser::EpisodePathMeta;
use super::{LibraryEvent, MediaLibrary};

impl MediaLibrary {
    pub async fn import_episode(&self, path: &Utf8Path) -> eyre::Result<()> {
        let Some(EpisodePathMeta {
            show_name,
            show_path,
            season,
            episode,
            name,
        }) = self.parser().parse_episode_path(path)
        else {
            return Ok(());
        };

        tracing::info!(%path, "importing episode");

        let name = match name {
            Some(name) => Cow::Borrowed(name),
            None => Cow::Owned(format!("S{season:02}E{episode:02}")),
        };

        let show_id = self.create_show_if_missing(show_name, show_path).await?;
        let season_id = self.create_season_if_missing(show_id, season).await?;
        let episode_id = self
            .create_episode_if_missing(show_id, season_id, season, episode, &name)
            .await?;

        self.create_video_file(path, episode_id)
            .await
            .wrap_err_with(|| format!("failed to insert video file for '{path}'"))
    }

    /// Adds a new show
    async fn create_show_if_missing(&self, name: &str, path: &Utf8Path) -> eyre::Result<i64> {
        let mut conn = self.db.acquire().await?;

        if let Some(id) = get_show_id_by_path(&mut conn, path).await? {
            return Ok(id);
        }

        tracing::info!(name, %path, "adding show");

        let mut transaction = conn.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type, name, metadata_provider)
            VALUES (?, ?, ?)
        ";

        let show_id: i64 = sqlx::query(sql)
            .bind(MediaItemType::Show)
            .bind(name)
            .bind(MetadataProvider::Tmdb)
            .execute(&mut *transaction)
            .await?
            .last_insert_rowid();

        let sql = "
            INSERT INTO indexed_paths (item_id, path)
            VALUES (?, ?)
        ";

        sqlx::query(sql)
            .bind(show_id)
            .bind(path)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::MediaAdded(MediaItemType::Show, show_id));

        Ok(show_id)
    }

    /// Adds a new season
    async fn create_season_if_missing(
        &self,
        show_id: i64,
        season_number: u32,
    ) -> eyre::Result<i64> {
        let mut conn = self.db.acquire().await?;

        if let Some(id) = get_season_id(&mut conn, show_id, season_number).await? {
            return Ok(id);
        }

        tracing::info!(show_id, season_number, "adding season");

        let mut transaction = conn.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type, name, parent_id, parent_index, metadata_provider)
            VALUES (?, ?, ?, ?, ?)
        ";

        let id: i64 = sqlx::query(sql)
            .bind(MediaItemType::Season)
            .bind(format!("Season {season_number}"))
            .bind(show_id)
            .bind(season_number)
            .bind(MetadataProvider::Tmdb)
            .execute(&mut *transaction)
            .await?
            .last_insert_rowid();

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::MediaAdded(MediaItemType::Season, id));

        Ok(id)
    }

    /// Adds a new episode
    async fn create_episode_if_missing(
        &self,
        show_id: i64,
        season_id: i64,
        season_number: u32,
        episode_number: u32,
        name: &str,
    ) -> eyre::Result<i64> {
        let mut conn = self.db.acquire().await?;

        if let Some(id) = get_episode_id(&mut conn, season_id, episode_number).await? {
            return Ok(id);
        }

        tracing::info!(show_id, season_number, episode_number, "adding episode");

        let mut transaction = conn.begin().await?;

        let sql = "
            INSERT INTO media_items (item_type, name, parent_id, parent_index, grandparent_id, grandparent_index, metadata_provider)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        ";

        let id = sqlx::query(sql)
            .bind(MediaItemType::Episode)
            .bind(name)
            .bind(season_id)
            .bind(episode_number)
            .bind(show_id)
            .bind(season_number)
            .bind(MetadataProvider::Tmdb)
            .execute(&mut *transaction)
            .await?
            .last_insert_rowid();

        transaction.commit().await?;

        let _ = self
            .notifier
            .send(LibraryEvent::MediaAdded(MediaItemType::Episode, id));

        Ok(id)
    }

    /// Validates the tv shows stored in the database
    ///
    /// This will delete episodes that don't have any files, seasons that
    /// don't have any episodes and shows that don't have any seasons.
    pub(super) async fn validate_shows(&self) -> eyre::Result<()> {
        tracing::info!("validating shows");

        let mut conn = self.db.acquire_write().await?;

        let sql = sql::select("media_items AS e")
            .columns(&["e.id", "e.grandparent_index", "e.parent_index", "s.name"])
            .joins(&[Join::inner("media_items AS s ON e.grandparent_id = s.id")])
            .condition("e.item_type = ? AND e.id NOT IN (SELECT item_id FROM video_files)")
            .to_sql();

        let episodes: Vec<(i64, u32, u32, String)> = sqlx::query_as(&sql)
            .bind(MediaItemType::Episode)
            .fetch_all(&mut *conn)
            .await?;

        for (id, season, episode, show_name) in episodes {
            tracing::info!(id, season, episode, show_name, "removing episode");
            self.remove_item(&mut conn, id, MediaItemType::Episode)
                .await?;
        }

        self.remove_empty_collections(&mut conn).await
    }

    async fn remove_empty_collections(&self, conn: &mut WriteConnection) -> eyre::Result<()> {
        // Don't use a subquery here - it doesn't seem to work. e.g.
        // SELECT * FROM media_items WHERE item_type = 3 AND id NOT IN (SELECT parent_id
        // FROM media_items) returns an empty result set despite orphan
        // seasons existing

        let sql = "
            SELECT s.id, s.parent_index, sh.name FROM media_items AS s
            JOIN media_items AS sh ON s.parent_id = sh.id
            LEFT JOIN media_items AS e ON s.id = e.parent_id
            WHERE s.item_type = ? AND e.id IS NULL
        ";

        let seasons: Vec<(i64, u32, String)> = sqlx::query_as(sql)
            .bind(MediaItemType::Season)
            .fetch_all(&mut *conn)
            .await?;

        for (id, season, show_name) in seasons {
            tracing::info!(id, season, show_name, "removing season");
            self.remove_item(&mut *conn, id, MediaItemType::Season)
                .await?;
        }

        let sql = "
            SELECT sh.id, sh.name FROM media_items AS sh
            LEFT JOIN media_items AS se
            ON sh.id = se.parent_id
            WHERE sh.item_type = ? AND se.id IS NULL
        ";

        let shows: Vec<(i64, String)> = sqlx::query_as(sql)
            .bind(MediaItemType::Show)
            .fetch_all(&mut *conn)
            .await?;

        for (id, name) in shows {
            tracing::info!(id, name, "removing show");
            self.remove_item(&mut *conn, id, MediaItemType::Show)
                .await?;
        }

        Ok(())
    }
}

async fn get_show_id_by_path(
    conn: &mut ReadConnection,
    path: &Utf8Path,
) -> eyre::Result<Option<i64>> {
    let sql = sql::select("indexed_paths")
        .columns(&["item_id"])
        .joins(&[Join::inner("shows ON shows.id = item_id")])
        .condition("path = ?")
        .to_sql();

    let id = sqlx::query_scalar(&sql)
        .bind(path)
        .fetch_optional(conn)
        .await?;

    Ok(id)
}

async fn get_season_id(
    conn: &mut ReadConnection,
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
        .fetch_optional(conn)
        .await?;

    Ok(id)
}

async fn get_episode_id(
    conn: &mut ReadConnection,
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
        .fetch_optional(conn)
        .await?;

    Ok(id)
}
