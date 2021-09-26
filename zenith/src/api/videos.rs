use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request};

use crate::db::Db;

use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/videos/:id").get(get_video_content);
}

#[endpoint]
async fn get_video_content(req: &mut Request) -> eyre::Result<()> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let sql = "
        SELECT path
        FROM video_files
        WHERE item_id = ?
    ";

    let path: String = sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .or_not_found("video not found")?;

    req.respond_file(path).await?;

    Ok(())
}
