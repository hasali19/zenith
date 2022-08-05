use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_files::{FileRequest, FileResponse};
use speq::axum::{delete, get};

use crate::api::error::bad_request;
use crate::api::ApiResult;
use crate::db::{self, Db};

use super::error::not_found;
use super::ext::OptionExt;

#[get("/subtitles/:id")]
#[path(i64)]
#[response(status = 200)]
async fn get_subtitle(
    id: Path<i64>,
    file: FileRequest,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let subtitle = db::subtitles::get_by_id(&mut conn, *id)
        .await?
        .or_not_found("subtitle not found")?;

    let res = match subtitle.path {
        Some(path) => FileResponse::from_request(file, path).await?,
        None => return Err(not_found("no file exists for this subtitle")),
    };

    Ok(res)
}

#[delete("/subtitles/:id")]
#[path(i64)]
#[response(status = 200)]
pub async fn delete_subtitle(id: Path<i64>, db: Extension<Db>) -> ApiResult<impl IntoResponse> {
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

    Ok(StatusCode::OK)
}
