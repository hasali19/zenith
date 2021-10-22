use atium::responder::File;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request, Responder};

use crate::db::{self, Db};

use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/videos/:id").get(get_video_content);
}

#[endpoint]
async fn get_video_content(req: &mut Request) -> eyre::Result<impl Responder> {
    let id: i64 = req.param("id")?;

    let db: &Db = req.ext().unwrap();
    let mut conn = db.acquire().await?;

    let info = db::videos::get_basic_info(&mut conn, id)
        .await?
        .or_not_found("video not found")?;

    Ok(File::open(info.path).await?)
}
