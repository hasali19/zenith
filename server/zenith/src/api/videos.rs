use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum_codegen::get;
use axum_files::{FileRequest, FileResponse};

use crate::api::ApiResult;
use crate::db::{self, Db};

use super::ext::OptionExt;

#[get("/videos/:id")]
#[path(i64)]
#[response(status = 200)]
pub async fn get_video_content(
    id: Path<i64>,
    file: FileRequest,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let info = db::videos::get_basic_info(&mut conn, *id)
        .await?
        .or_not_found("video not found")?;

    Ok(FileResponse::from_request(file, info.path).await?)
}
