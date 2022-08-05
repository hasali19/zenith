use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::extract::{Extension, Multipart};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use speq::axum::post;
use tokio::fs::File;
use tokio::io::BufWriter;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::api::error::bad_request;
use crate::api::ext::OptionExt;
use crate::api::ApiResult;
use crate::config::Config;
use crate::db::subtitles::NewSubtitle;
use crate::db::Db;
use crate::{db, subtitles, util};

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
}

// TODO: Support specifying multipart requests
#[post("/import/subtitle")]
#[response(status = 200)]
pub async fn import_subtitle(
    mut multipart: Multipart,
    config: Extension<Arc<Config>>,
    db: Extension<Db>,
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

    let subtitles_dir = config.subtitles.path.join(data.video_id.to_string());
    let src_path = {
        let field = multipart
            .next_field()
            .await
            .map_err(bad_request)?
            .or_bad_request("upload import source specified but no file found in request")?;

        let content_type = field
            .content_type()
            .or_bad_request("missing content type for file upload")?;

        if !Path::new("data/tmp").exists() {
            std::fs::create_dir_all("data/tmp")?;
        }

        let id = Uuid::new_v4();
        let src_path = format!("data/tmp/{id}.vtt");
        let file = BufWriter::new(File::create(&src_path).await?);

        match content_type {
            // vtt subtitles can be directly written to the file
            "text/vtt" => util::copy_stream(field, file).await?,
            // srt subtitles need to be converted first
            "application/x-subrip" => subtitles::convert(&config, field, file).await?,
            _ => return Err(bad_request("unsupported subtitle content-type")),
        }

        PathBuf::from(src_path)
    };

    let dst_name = Uuid::new_v4().to_string();
    let dst_path = subtitles_dir.join(dst_name).with_extension("vtt");
    if dst_path.exists() {
        return Err(bad_request(format!("{dst_path:?} already exists")));
    }

    let mut transaction = db.begin().await?;
    let subtitles = NewSubtitle {
        video_id: data.video_id,
        stream_index: None,
        path: dst_path.to_str(),
        title: data.title.as_deref(),
        language: data.language.as_deref(),
    };

    db::subtitles::insert(&mut transaction, &subtitles).await?;

    if !subtitles_dir.exists() {
        std::fs::create_dir_all(&subtitles_dir)?;
    }

    tracing::info!("copying {src_path:?} to {dst_path:?}");
    std::fs::copy(&src_path, &dst_path)?;

    transaction.commit().await?;

    Ok(StatusCode::OK)
}
