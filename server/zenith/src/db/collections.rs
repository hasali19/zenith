use serde::Serialize;
use speq::Reflect;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Acquire, Arguments, FromRow, SqliteConnection};

use crate::sql;

#[derive(Serialize, Reflect)]
pub struct CollectionUserData {
    pub unwatched: u32,
}

#[derive(FromRow)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub overview: Option<String>,
    pub poster: Option<String>,
}

pub async fn get_all(conn: &mut SqliteConnection) -> eyre::Result<Vec<Collection>> {
    Ok(sqlx::query_as("SELECT id, name, overview, (SELECT m.poster FROM collections_media_items AS mc JOIN media_items AS m ON m.id = mc.item_id WHERE mc.collection_id = c.id ORDER BY mc.idx LIMIT 1) AS poster FROM collections AS c")
        .fetch_all(conn)
        .await?)
}

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<Collection>> {
    Ok(
        sqlx::query_as("SELECT id, name, overview, (SELECT m.poster FROM collections_media_items AS mc JOIN media_items AS m ON m.id = mc.item_id WHERE mc.collection_id = c.id ORDER BY mc.idx LIMIT 1) AS poster FROM collections AS c WHERE id = ?")
            .bind(id)
            .fetch_optional(conn)
            .await?,
    )
}

pub struct NewCollection<'a> {
    pub name: &'a str,
}

pub async fn create(
    conn: &mut SqliteConnection,
    data: NewCollection<'_>,
) -> eyre::Result<Collection> {
    let id = sqlx::query_scalar("INSERT INTO collections (name) VALUES (?) RETURNING (id)")
        .bind(data.name)
        .fetch_one(conn)
        .await?;

    Ok(Collection {
        id,
        name: data.name.to_owned(),
        overview: None,
        poster: None,
    })
}

pub struct UpdateCollection<'a> {
    pub name: &'a str,
    pub overview: Option<&'a str>,
}

pub async fn update(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateCollection<'_>,
) -> eyre::Result<()> {
    sqlx::query("UPDATE collections SET name = ?, overview = ? WHERE id = ?")
        .bind(data.name)
        .bind(data.overview)
        .bind(id)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn remove(conn: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    let mut tx = conn.begin().await?;

    sqlx::query("DELETE FROM collections_media_items WHERE collection_id = ?")
        .bind(id)
        .execute(&mut tx)
        .await?;

    sqlx::query("DELETE FROM collections WHERE id = ?")
        .execute(&mut tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn set_items(conn: &mut SqliteConnection, id: i64, items: &[i64]) -> eyre::Result<()> {
    let mut tx = conn.begin().await?;

    let mut args = SqliteArguments::default();
    args.add(id);

    for (i, item_id) in items.iter().enumerate() {
        args.add(item_id);
        sqlx::query("INSERT INTO collections_media_items VALUES (?1, ?2, ?3) ON CONFLICT DO UPDATE SET idx = ?3")
            .bind(id)
            .bind(item_id)
            .bind(i as i64)
            .execute(&mut tx)
            .await?;
    }

    let placeholders = sql::Placeholders(items.len());

    #[rustfmt::skip]
    let sql = format!("
        DELETE FROM collections_media_items
        WHERE collection_id = ? AND item_id NOT IN ({placeholders})
    ");

    sqlx::query_with(&sql, args).execute(&mut tx).await?;

    tx.commit().await?;

    Ok(())
}
