use std::str::FromStr;

use mime::Mime;
use zenith_http::headers::{AccessControlAllowOrigin, ContentType};
use zenith_http::{App, Body, Request, Response};

use crate::AppState;

use super::{ApiError, ApiResult};

pub fn configure(app: &mut App<AppState>) {
    app.get("/api/hls/:id/main.m3u8", get_hls_playlist);
    app.get("/api/hls/:id/:file_name", get_hls_segment);
    app.put("/api/hls/receiver/:file_name", receive_hls_segment);
}

async fn get_hls_playlist(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let playlist = state
        .transcoder
        .generate_playlist(id)
        .await
        .ok_or_else(ApiError::not_found)?;

    let mime_type = Mime::from_str("application/mpegURL").unwrap();

    Ok(Response::new()
        .with_header(ContentType::from(mime_type))
        .with_header(AccessControlAllowOrigin::ANY)
        .with_body(Body::from(playlist)))
}

async fn get_hls_segment(state: AppState, req: Request) -> ApiResult {
    let id: i64 = req
        .param("id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let segment: u32 = req
        .param("file_name")
        .and_then(|f| f.split('.').next())
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    let mime_type = Mime::from_str("video/mp2t").unwrap();
    let bytes = state
        .transcoder
        .request_segment(id, segment)
        .await
        .unwrap()
        .ok_or_else(ApiError::not_found)?;

    Ok(Response::new()
        .with_header(ContentType::from(mime_type))
        .with_header(AccessControlAllowOrigin::ANY)
        .with_body(Body::from(bytes)))
}

async fn receive_hls_segment(state: AppState, mut req: Request) -> ApiResult {
    let segment: u32 = req
        .param("file_name")
        .and_then(|f| f.split('.').next())
        .and_then(|v| v.parse().ok())
        .ok_or_else(ApiError::bad_request)?;

    state.transcoder.receive_segment(segment, req.body()).await;

    Ok(Response::new())
}
