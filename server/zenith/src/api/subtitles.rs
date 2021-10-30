use actix_files::NamedFile;
use actix_web::web::Path;
use actix_web::{delete, route, HttpResponse, Responder};

use crate::api::error::bad_request;
use crate::api::ApiResult;
use crate::db::{self, Db};

use super::error::not_found;
use super::ext::OptionExt;

#[route("/subtitles/{id}", method = "HEAD", method = "GET")]
async fn get_subtitle(id: Path<i64>, db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let subtitle = db::subtitles::get_by_id(&mut conn, *id)
        .await?
        .or_not_found("subtitle not found")?;

    let res = match subtitle.path {
        Some(path) => NamedFile::open(path)?,
        None => return Err(not_found("no file exists for this subtitle")),
    };

    Ok(res)
}

#[delete("/subtitles/{id}")]
pub async fn delete_subtitle(id: Path<i64>, db: Db) -> ApiResult<impl Responder> {
    let mut conn = db.acquire().await?;

    let subtitle = db::subtitles::get_by_id(&mut conn, *id)
        .await?
        .or_not_found("subtitle not found")?;

    if subtitle.stream_index.is_some() {
        return Err(bad_request("embedded subtitles cannot be deleted"));
    }

    db::subtitles::delete(&mut conn, *id).await?;

    if let Some(path) = subtitle.path {
        tokio::fs::remove_file(path).await?;
    }

    Ok(HttpResponse::Ok())
}
