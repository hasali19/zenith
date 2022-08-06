use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;
use axum_files::{FileRequest, FileResponse};
use sha2::{Digest, Sha256};
use speq::axum::get;

use crate::config::Config;
use crate::db::media::MediaImageType;
use crate::db::{self, Db};

use super::ext::OptionExt;
use super::ApiResult;

#[get("/items/:id/images/:type")]
#[path(i64, MediaImageType)]
#[response(status = 200)]
#[response(status = 404)]
pub async fn get_image(
    Path((id, img_type)): Path<(i64, MediaImageType)>,
    file: FileRequest,
    Extension(config): Extension<Arc<Config>>,
    Extension(db): Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let item = db::items::get(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let url = item
        .image(img_type)
        .or_not_found("item does not have image of requested type")?;

    let img_path = get_img_path(url, &config.paths.cache).await?;

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
