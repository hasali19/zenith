use serde::Deserialize;
use zenith_http::{App, Request, Response};

use crate::AppState;

use super::{ApiError, ApiResult};

pub fn configure(app: &mut App<AppState>) {
    app.post("/api/progress/:id", update_progress);
}

#[derive(Deserialize)]
struct ProgressUpdate {
    position: f64,
}

async fn update_progress(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let data: ProgressUpdate = req
        .query()
        .map_err(|e| ApiError::bad_request().body(e.to_string()))?;

    let mut conn = state.db.acquire().await?;

    let sql = "
        SELECT duration
        FROM video_files
        WHERE item_id = ?
    ";

    let duration: f64 = sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let sql = "
        INSERT INTO user_item_data (item_id, position, is_watched)
        VALUES (?, ?, ?)
        ON CONFLICT (item_id) DO UPDATE
        SET position = excluded.position,
            is_watched = is_watched OR excluded.is_watched
    ";

    sqlx::query(sql)
        .bind(id)
        .bind(data.position)
        .bind((data.position / duration) >= 0.9)
        .execute(&mut conn)
        .await?;

    Ok(Response::new())
}
