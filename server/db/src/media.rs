use sqlx::Type;

use crate::ReadConnection;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Type)]
#[repr(i32)]
pub enum MediaItemType {
    Movie = 1,
    Show = 2,
    Season = 3,
    Episode = 4,
}

impl MediaItemType {
    pub fn is_video(&self) -> bool {
        matches!(self, MediaItemType::Movie | MediaItemType::Episode)
    }
}

pub async fn get_item_type(
    conn: &mut ReadConnection,
    id: i64,
) -> eyre::Result<Option<MediaItemType>> {
    sqlx::query_scalar("SELECT item_type FROM media_items WHERE id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await
        .map_err(|e| e.into())
}

#[derive(Debug, Type)]
#[sqlx(rename_all = "snake_case")]
pub enum MetadataProvider {
    Tmdb,
}
