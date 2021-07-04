use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::db::Db;

#[derive(Deserialize)]
pub struct ProgressUpdate {
    position: f64,
}

/// POST /api/progress/{id}
pub async fn update_progress(
    req: HttpRequest,
    path: web::Path<(i64,)>,
    query: web::Query<ProgressUpdate>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();
    let query = query.into_inner();

    let db: &Db = req.app_data().unwrap();
    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;

    let sql = "
        SELECT duration
        FROM video_files
        WHERE item_id = ?
    ";

    let duration: f64 = sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound(""))?;

    let sql = "
        INSERT INTO user_item_data (item_id, position, is_watched)
        VALUES (?, ?, ?)
        ON CONFLICT (item_id) DO UPDATE
        SET position = excluded.position,
            is_watched = is_watched OR excluded.is_watched
    ";

    sqlx::query(sql)
        .bind(id)
        .bind(query.position)
        .bind((query.position / duration) >= 0.9)
        .execute(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}
