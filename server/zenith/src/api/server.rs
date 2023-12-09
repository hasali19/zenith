use axum::body::Body;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use speq::axum::post;
use time::OffsetDateTime;

use crate::Db;

use super::ApiResult;

#[post("/server/db/backup")]
async fn backup_db(db: Extension<Db>) -> ApiResult<impl IntoResponse> {
    let timestamp = OffsetDateTime::now_utc().unix_timestamp();
    let content_disposition = format!("attachment; filename=\"zenith_{timestamp}.db\"");

    let res = Response::builder()
        .header("content-type", "application/vnd.sqlite3")
        .header("content-disposition", content_disposition)
        .body(Body::from(db.backup().await?))
        .unwrap();

    Ok(res)
}
