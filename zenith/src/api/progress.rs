use atium::query::QueryRequestExt;
use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request};
use serde::Deserialize;

use crate::db::Db;

use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/progress/:id").post(update_progress);
}

#[derive(Deserialize)]
struct ProgressUpdate {
    position: f64,
}

#[endpoint]
async fn update_progress(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;
    let query: ProgressUpdate = req.query()?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT duration
        FROM video_files
        WHERE item_id = ?
    ";

    let duration: f64 = sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .or_not_found("video not found")?;

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
        .await?;

    req.ok();

    Ok(())
}
