use uuid::Uuid;

use crate::{ReadConnection, WriteConnection, sql};

#[derive(Debug, sqlx::FromRow)]
pub struct Image {
    pub id: String,
    pub image_type: ImageType,
    pub source_type: ImageSourceType,
    pub source: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, sqlx::Type)]
#[repr(u8)]
pub enum ImageType {
    Poster = 1,
    Backdrop = 2,
    Thumbnail = 3,
    Profile = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, sqlx::Type)]
#[repr(u8)]
pub enum ImageSourceType {
    Tmdb = 1,
}

pub async fn get(conn: &mut ReadConnection, id: &str) -> eyre::Result<Option<Image>> {
    let sql = sql::select("images")
        .columns(&["id", "image_type", "source_type", "source"])
        .condition("id = ?")
        .to_sql();

    let image = sqlx::query_as(&sql)
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?;

    Ok(image)
}

pub async fn get_or_create(
    conn: &mut WriteConnection,
    image_type: ImageType,
    source_type: ImageSourceType,
    source: &str,
) -> eyre::Result<String> {
    let mut tx = conn.begin().await?;

    let sql = sql::select("images")
        .columns(&["id"])
        .condition("image_type = ? AND source_type = ? AND source = ?")
        .to_sql();

    let id = sqlx::query_scalar(&sql)
        .bind(image_type)
        .bind(source_type)
        .bind(source)
        .fetch_optional(&mut *tx)
        .await?;

    if let Some(id) = id {
        return Ok(id);
    }

    let id = Uuid::new_v4().to_string();
    let sql = sql::insert("images")
        .columns(&["id", "image_type", "source_type", "source"])
        .values(&["?", "?", "?", "?"])
        .to_sql();

    sqlx::query(&sql)
        .bind(&id)
        .bind(image_type)
        .bind(source_type)
        .bind(source)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(id)
}
