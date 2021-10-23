use actix_files::NamedFile;
use actix_web::web::Path;
use actix_web::{get, Responder};

use crate::api::ApiResult;
use crate::db::{self, Db};

use super::ext::OptionExt;

#[get("/videos/{id}")]
pub async fn get_video_content(id: Path<i64>, db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let info = db::videos::get_basic_info(&mut conn, *id)
        .await?
        .or_not_found("video not found")?;

    Ok(NamedFile::open(info.path)?)
}
