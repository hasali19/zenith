use std::borrow::Cow;
use std::fmt::Write;
use std::io;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use axum::Extension;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum_files::{FileRequest, FileResponse};
use camino::{Utf8Path, Utf8PathBuf};
use db::Db;
use db::images::{Image, ImageSourceType, ImageType};
use eyre::eyre;
use serde::Deserialize;
use serde_qs::axum::QsQuery;
use sha2::{Digest, Sha256};
use speq::axum::get;
use speq::{Reflect, StatusCode};
use tokio::fs;

use crate::config::Config;

use super::ApiResult;
use super::error::ApiError;
use super::ext::OptionExt;

#[derive(Deserialize, Reflect)]
pub struct ImageQuery {
    width: Option<u32>,
}

#[get("/images/{id}")]
pub async fn get_image(
    Path(id): Path<String>,
    QsQuery(query): QsQuery<ImageQuery>,
    file: FileRequest,
    Extension(config): Extension<Arc<Config>>,
    Extension(db): Extension<Db>,
) -> ApiResult<impl IntoResponse> {
    let mut conn = db.acquire().await?;

    let image_path = config
        .paths
        .cache
        .join("images")
        .join(&id[..2])
        .join(&id)
        .join(build_file_name(query.width).as_ref())
        .with_extension("jpg");

    if fs::try_exists(&image_path).await? {
        return Ok(FileResponse::from_request(file, &image_path).await?);
    }

    let image = db::images::get(&mut conn, &id)
        .await?
        .or_not_found("image not found")?;

    if image.source_type != ImageSourceType::Tmdb {
        return Err(ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            inner: Arc::new(eyre!(
                "only tmdb images are currently supported by this endpoint"
            )),
        });
    }

    download_tmdb_image(&image, &query, &image_path).await?;

    Ok(FileResponse::from_request(file, &image_path).await?)
}

fn build_file_name(width: Option<u32>) -> Cow<'static, str> {
    if let Some(width) = width {
        Cow::Owned(format!("w{width}"))
    } else {
        Cow::Borrowed("original")
    }
}

async fn download_tmdb_image(
    image: &Image,
    query: &ImageQuery,
    destination: &Utf8Path,
) -> eyre::Result<()> {
    // TODO: query available sizes from tmdb api
    let available_sizes = available_tmdb_sizes(image.image_type);

    let actual_size = query.width.and_then(|requested_size| {
        available_sizes
            .iter()
            .copied()
            .find(|&size| size >= requested_size)
    });

    let url = build_tmdb_url(&image.source, actual_size);

    download_image(&url, destination).await?;

    Ok(())
}

fn available_tmdb_sizes(image_type: ImageType) -> &'static [u32] {
    match image_type {
        ImageType::Poster => &[92, 154, 185, 342, 500, 780],
        ImageType::Backdrop => &[300, 780, 1280],
        ImageType::Thumbnail => &[92, 185, 300, 780],
        ImageType::Profile => &[45, 185],
    }
}

fn build_tmdb_url(path: &str, size: Option<u32>) -> String {
    let mut url = String::from("https://image.tmdb.org/t/p/");
    if let Some(size) = size {
        write!(url, "w{size}").unwrap();
    } else {
        url += "original";
    }
    url += path;
    url
}

async fn download_image(url: &str, destination: &Utf8Path) -> eyre::Result<()> {
    tracing::info!(path = %destination, url = %url, "downloading image");

    let res = reqwest::get(url).await?.error_for_status()?;
    let bytes = res.bytes().await?;

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).await?;
    }

    fs::write(destination, bytes).await?;

    Ok(())
}

#[derive(Deserialize, Reflect)]
#[serde(rename_all = "snake_case")]
pub enum MediaImageType {
    Poster,
    Backdrop,
    Thumbnail,
}

#[get("/items/{id}/images/{type}")]
#[response(status = 200)]
#[response(status = 404)]
pub async fn get_image_for_item(
    Path((id, img_type)): Path<(i64, MediaImageType)>,
    QsQuery(query): QsQuery<ImageQuery>,
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
        None => match img_type {
            MediaImageType::Poster => "w342",
            MediaImageType::Backdrop | MediaImageType::Thumbnail => "original",
        },
    };

    let item = db::items::get(&mut conn, id)
        .await?
        .or_not_found("media item not found")?;

    let img = match img_type {
        MediaImageType::Poster => item.poster,
        MediaImageType::Backdrop => item.backdrop,
        MediaImageType::Thumbnail => item.thumbnail,
    };

    let img_id = img.or_not_found("item does not have image of requested type")?;
    let img = db::images::get(&mut conn, &img_id)
        .await?
        .or_not_found("image not found")?;

    if img.source_type != ImageSourceType::Tmdb {
        return Err(ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            inner: Arc::new(eyre!("only tmdb images are supported by this endpoint")),
        });
    }

    let src = &img.source;
    let url = format!("https://image.tmdb.org/t/p/{size}{src}");

    let img_path = get_img_path(&url, &config.paths.cache).await?;

    Ok(FileResponse::from_request(file, img_path).await?)
}

async fn get_img_path(url: &str, cache_dir: &Utf8Path) -> ApiResult<Utf8PathBuf> {
    let hash = {
        let mut hasher = Sha256::new();
        hasher.update(url);
        format!("{:x}", hasher.finalize())
    };

    let cached_path = cache_dir
        .join("images")
        .join(&hash[..2])
        .join(&hash)
        .with_extension("jpg");

    if should_refetch(&cached_path)? {
        tracing::info!(
            path = %cached_path,
            url = %url,
            "image is out of date",
        );

        let bytes = reqwest::get(url)
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .bytes()
            .await
            .unwrap();

        std::fs::create_dir_all(cached_path.parent().unwrap())?;
        std::fs::write(&cached_path, bytes)?;
    }

    Ok(cached_path)
}

fn should_refetch(path: &Utf8Path) -> io::Result<bool> {
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
