use sqlx::SqliteConnection;

use crate::media::MediaImage;
use crate::sql;

pub struct NewPerson<'a> {
    pub tmdb_id: Option<i32>,
    pub name: &'a str,
    pub profile: Option<MediaImage<'a>>,
}

pub async fn create(conn: &mut SqliteConnection, person: NewPerson<'_>) -> eyre::Result<i64> {
    let sql = sql::insert("people")
        .columns(&["tmdb_id", "name", "profile"])
        .values(&["?", "?", "?"])
        .returning(&["id"])
        .to_sql();

    let id = sqlx::query_scalar(&sql)
        .bind(person.tmdb_id)
        .bind(person.name)
        .bind(person.profile.map(|img| img.to_string()))
        .fetch_one(conn)
        .await?;

    Ok(id)
}

pub async fn get_by_tmdb_id_or_create(
    conn: &mut SqliteConnection,
    tmdb_id: i32,
    person: NewPerson<'_>,
) -> eyre::Result<i64> {
    let sql = sql::select("people")
        .columns(&["id"])
        .condition("tmdb_id = ?")
        .to_sql();

    let id = sqlx::query_scalar(&sql)
        .bind(tmdb_id)
        .fetch_optional(&mut *conn)
        .await?;

    if let Some(id) = id {
        return Ok(id);
    }

    create(
        conn,
        NewPerson {
            tmdb_id: Some(tmdb_id),
            name: person.name,
            profile: person.profile,
        },
    )
    .await
}
