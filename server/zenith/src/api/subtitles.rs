use std::process::Stdio;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_files::{FileRequest, FileResponse};
use db::subtitles::Subtitle;
use db::Db;
use eyre::Context;
use hyper::header::CONTENT_TYPE;
use serde::Deserialize;
use serde_qs::axum::QsQuery;
use speq::axum::{delete, get};
use speq::Reflect;
use tokio::process::Command;

use crate::api::error::bad_request;
use crate::api::ApiResult;
use crate::config::Config;

use super::error::not_found;
use super::ext::OptionExt;

#[derive(Deserialize, Reflect)]
#[serde(rename_all = "lowercase")]
enum Format {
    WebVTT,
}

#[derive(Deserialize, Reflect)]
struct GetSubtitleParams {
    format: Option<Format>,
}

#[get("/subtitles/:id")]
#[path(i64)]
#[response(status = 200)]
async fn get_subtitle(
    id: Path<i64>,
    #[query] QsQuery(params): QsQuery<GetSubtitleParams>,
    file: FileRequest,
    db: Extension<Db>,
    config: Extension<Arc<Config>>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let subtitle = db::subtitles::get_by_id(&mut conn, *id)
        .await?
        .or_not_found("subtitle not found")?;

    let path = subtitle
        .path
        .as_deref()
        .ok_or_else(|| not_found("no file exists for this subtitle"))?;

    let conversion_params = if let Some(requested_format) = params.format {
        get_conversion_if_needed(&subtitle, requested_format)
    } else {
        None
    };

    let res = if let Some((requested_format, response_content_type)) = conversion_params {
        let ffmpeg_path = &config.transcoding.ffmpeg_path;
        let content = convert_subtitle(ffmpeg_path, path.as_str(), requested_format).await?;
        Response::builder()
            .header(CONTENT_TYPE, response_content_type)
            .body(Body::from(content))
            .wrap_err("failed to create response")?
    } else {
        FileResponse::from_request(file, path)
            .await?
            .into_response()
    };

    Ok(res)
}

fn get_conversion_if_needed(subtitle: &Subtitle, requested_format: Format) -> Option<(&str, &str)> {
    let (requested_format, response_content_type) = match requested_format {
        Format::WebVTT => ("webvtt", "text/vtt"),
    };

    if subtitle.format.as_deref() == Some(requested_format) {
        None
    } else {
        Some((requested_format, response_content_type))
    }
}

async fn convert_subtitle(
    ffmpeg_path: &str,
    subtitle_path: &str,
    requested_format: &str,
) -> eyre::Result<Vec<u8>> {
    let output = Command::new(ffmpeg_path)
        .args(["-i", subtitle_path])
        .args(["-f", requested_format])
        .arg("-")
        .stdout(Stdio::piped())
        .output()
        .await
        .wrap_err("ffmpeg command failed")?;

    Ok(output.stdout)
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
