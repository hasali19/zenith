use atium::query::QueryRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request, Responder, StatusCode};
use serde::Deserialize;

use crate::db::videos::UpdateVideoUserData;
use crate::db::{self, Db};

use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/progress/:id").post(update_progress);
}

#[derive(Deserialize)]
struct ProgressUpdate {
    position: f64,
}

#[endpoint]
async fn update_progress(req: &mut Request) -> eyre::Result<impl Responder> {
    let id: i64 = req.param("id")?;
    let query: ProgressUpdate = req.query()?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let duration = db::videos::get_basic_info(&mut conn, id)
        .await?
        .or_not_found("video not found")?
        .duration;

    let data = UpdateVideoUserData {
        position: Some(query.position),
        is_watched: Some((query.position / duration) >= 0.9),
    };

    db::videos::update_user_data(&mut conn, id, data).await?;

    Ok(StatusCode::OK)
}
