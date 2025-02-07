use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum_files::{FileRequest, FileResponse};
use camino::Utf8Path;
use db::Db;
use serde::Deserialize;
use serde_qs::axum::QsQuery;
use speq::axum::get;
use speq::Reflect;

use crate::api::ApiResult;

use super::ext::OptionExt;

#[derive(Deserialize, Reflect)]
pub struct GetVideoContentQuery {
    #[serde(default)]
    attachment: bool,
}

#[get("/videos/{id}")]
#[path(i64)]
#[response(status = 200)]
pub async fn get_video_content(
    id: Path<i64>,
    #[query] query: QsQuery<GetVideoContentQuery>,
    file: FileRequest,
    db: Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let info = db::video_files::get(&mut conn, *id)
        .await?
        .or_not_found("video not found")?;

    let path = Utf8Path::new(&info.path);
    let filename = path.file_name().unwrap();
    let mut res = FileResponse::from_request(file, &info.path).await?;

    if query.attachment {
        res = res.with_content_disposition(
            format!("attachment; filename=\"{filename}\"")
                .parse()
                .unwrap(),
        );
    }

    Ok(res)
}
