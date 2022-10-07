use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;
use axum_files::{FileRequest, FileResponse};
use serde::Deserialize;
use serde_qs::axum::QsQuery;
use sha2::{Digest, Sha256};
use speq::axum::get;
use speq::Reflect;

use crate::config::Config;
use crate::db::media::MediaImageType;
use crate::db::{self, Db};

use super::ext::OptionExt;
use super::ApiResult;

#[derive(Deserialize, Reflect)]
pub struct ImageQuery {
    width: Option<u32>,
}

#[get("/items/:id/images/:type")]
#[path(i64, MediaImageType)]
#[response(status = 200)]
#[response(status = 404)]
pub async fn get_image(
    Path((id, img_type)): Path<(i64, MediaImageType)>,
    #[query] QsQuery(query): QsQuery<ImageQuery>,
    file: FileRequest,
    Extension(config): Extension<Arc<Config>>,
    Extension(db): Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    // TODO: query sizes from tmdb api dynamically
    let size = match query.width {
        Some(width) => match img_type {
            MediaImageType::Poster => match width {
                0..=92 => "w92",
                93..=154 => "w154",
                155..=185 => "w185",
                186..=342 => "w342",
                343..=500 => "w500",
                _ => "original",
            },
            MediaImageType::Backdrop => match width {
                0..=300 => "w300",
                301..=780 => "w780",
                781..=1280 => "w1280",
                _ => "original",
            },
            MediaImageType::Thumbnail => match width {
                0..=92 => "w92",
                93..=185 => "w185",
                186..=300 => "w300",
                _ => "original",
            },
        },
        None => "original",
    };

    let item = db::items::get(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let url = item
        .image(img_type)
        .or_not_found("item does not have image of requested type")?;

    // FIXME: super hacky - should use stored db values directly rather than result of utils::get_image_url
    let url = match img_type {
        MediaImageType::Poster => url.replacen("w342", size, 1),
        MediaImageType::Backdrop | MediaImageType::Thumbnail => url.replacen("original", size, 1),
    };

    let img_path = get_img_path(&url, &config.paths.cache).await?;

    Ok(FileResponse::from_request(file, img_path).await?)
}

async fn get_img_path(url: &str, cache_dir: &std::path::Path) -> ApiResult<PathBuf> {
    let hash = {
        let mut hasher = Sha256::new();
        hasher.update(&url);
        format!("{:x}", hasher.finalize())
    };

    let cached_path = cache_dir
        .join("images")
        .join(&hash[..2])
        .join(&hash)
        .with_extension("jpg");

    if should_refetch(&cached_path)? {
        tracing::info!(
            path = %cached_path.display(),
            url = %url,
            "image is out of date",
        );
        let bytes = reqwest::get(url).await.unwrap().bytes().await.unwrap();
        std::fs::create_dir_all(cached_path.parent().unwrap())?;
        std::fs::write(&cached_path, bytes)?;
    }

    Ok(cached_path)
}

fn should_refetch(path: &std::path::Path) -> io::Result<bool> {
    match path.metadata() {
        Ok(metadata) => {
            let now = SystemTime::now();
            let a_week_ago = now - Duration::new(60 * 60 * 24 * 7, 0);
            Ok(metadata.modified()? < a_week_ago)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(true),
        Err(e) => Err(e),
    }
}
