use crate::sql::{self, OnConflict, UpdateList};
use crate::{ReadConnection, WriteConnection};

pub async fn get_all_ids(conn: &mut ReadConnection) -> eyre::Result<Vec<i64>> {
    sqlx::query_scalar("SELECT item_id FROM video_files")
        .fetch_all(conn)
        .await
        .map_err(Into::into)
}

pub struct VideoUserData {
    pub is_watched: bool,
    pub position: Option<f64>,
    pub last_watched_at: Option<i64>,
}

pub struct UpdateVideoUserData {
    pub is_watched: Option<bool>,
    pub position: Option<f64>,
    /// Whether to update the position_updated_at timestamp to the current time.
    pub set_position_updated: bool,
}

pub async fn update_user_data(
    conn: &mut WriteConnection,
    media_id: i64,
    user_id: i64,
    data: UpdateVideoUserData,
) -> eyre::Result<VideoUserData> {
    let mut columns = vec!["item_id", "user_id", "position", "is_watched"];
    let mut values = vec![
        "?1",
        "?2",
        "MAX(0, MIN(COALESCE(?3, 0), (SELECT duration FROM video_files WHERE item_id = ?1)))",
        "COALESCE(?4, 0)",
    ];

    let mut update_values = vec![
        "MAX(0, MIN(COALESCE(?3, position), (SELECT duration FROM video_files WHERE item_id = ?1)))",
        "COALESCE(?4, is_watched)",
    ];

    if data.set_position_updated {
        columns.push("position_updated_at");
        values.push("strftime('%s', 'now')");
        update_values.push("strftime('%s', 'now')");
    }

    let sql = sql::insert("media_item_user_data")
        .columns(&columns)
        .values(&values)
        .on_conflict(OnConflict::Update(
            UpdateList::new()
                .columns(&columns[2..])
                .values(&update_values),
        ))
        .returning(&[
            "CAST(position AS REAL)",
            "is_watched",
            "position_updated_at",
        ])
        .to_sql();

    let (position, is_watched, last_watched_at) = sqlx::query_as(&sql)
        .bind(media_id)
        .bind(user_id)
        .bind(data.position)
        .bind(data.is_watched)
        .fetch_one(conn)
        .await?;

    let user_data = VideoUserData {
        position,
        is_watched,
        last_watched_at,
    };

    Ok(user_data)
}
