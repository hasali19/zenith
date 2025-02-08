use std::sync::Arc;

use axum::extract::{Extension, Multipart};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use db::Db;
use db::subtitles::NewSubtitle;
use serde::Deserialize;
use speq::axum::post;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::api::ApiResult;
use crate::api::error::bad_request;
use crate::api::ext::OptionExt;
use crate::config::Config;
use crate::{subtitles, util};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ImportSource {
    Upload,
}

#[derive(Deserialize)]
pub struct ImportSubtitleRequest {
    #[serde(flatten)]
    data: ImportSubtitleRequestData,
}

#[derive(Deserialize)]
pub struct ImportSubtitleRequestData {
    video_id: i64,
    title: Option<String>,
    language: Option<String>,
    #[serde(default)]
    sdh: bool,
    #[serde(default)]
    forced: bool,
}

// TODO: Support specifying multipart requests
#[post("/import/subtitle")]
#[response(status = 200)]
pub async fn import_subtitle(
    config: Extension<Arc<Config>>,
    db: Extension<Db>,
    mut multipart: Multipart,
) -> ApiResult<impl IntoResponse> {
    let ImportSubtitleRequest { data } = {
        let mut field = multipart
            .next_field()
            .await
            .map_err(bad_request)?
            .or_bad_request("missing data field in mutipart body")?;

        if !matches!(field.name(), Some("data")) {
            return Err(bad_request("first field in multipart body must be 'data'"));
        }

        let mut bytes = vec![];
        while let Some(chunk) = field.next().await {
            bytes.extend_from_slice(chunk.map_err(bad_request)?.as_ref());
        }

        serde_json::from_slice(&bytes).map_err(bad_request)?
    };

    let subtitle_data = {
        let field = multipart
            .next_field()
            .await
            .map_err(bad_request)?
            .or_bad_request("upload import source specified but no file found in request")?;

        let content_type = field
            .content_type()
            .or_bad_request("missing content type for file upload")?;

        let mut out_buf = vec![];

        match content_type {
            // vtt subtitles can be directly written to the file
            "text/vtt" => util::copy_stream(field, &mut out_buf).await?,
            // srt subtitles need to be converted first
            "application/x-subrip" => subtitles::convert(&config, field, &mut out_buf).await?,
            _ => return Err(bad_request("unsupported subtitle content-type")),
        }

        out_buf
    };

    let subtitles_dir = config.subtitles.path.join(data.video_id.to_string());
    let dst_name = Uuid::new_v4().to_string();
    let dst_path = subtitles_dir.join(dst_name).with_extension("vtt");
    if dst_path.exists() {
        return Err(bad_request(format!("{dst_path:?} already exists")));
    }

    let mut transaction = db.begin_write().await?;
    let subtitles = NewSubtitle {
        video_id: data.video_id,
        stream_index: None,
        path: Some(&dst_path),
        title: data.title.as_deref(),
        language: data.language.as_deref(),
        format: Some("webvtt"),
        sdh: data.sdh,
        forced: data.forced,
    };

    db::subtitles::insert(&mut transaction, &subtitles).await?;

    if !subtitles_dir.exists() {
        tokio::fs::create_dir_all(&subtitles_dir).await?;
    }

    tracing::info!("writing subtitles to {dst_path:?}");
    tokio::fs::write(&dst_path, subtitle_data).await?;

    transaction.commit().await?;

    Ok(StatusCode::OK)
}
