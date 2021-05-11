use std::path::Path;
use std::sync::Arc;

use actix_http::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::config::Config;
use crate::db::Db;

use super::ApiResult;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ImportSource {
    Local { path: String },
}

#[derive(Deserialize)]
pub struct ImportShowRequest {
    name: String,
    episodes: Vec<ImportEpisodeRequest>,
}

pub async fn import_show(
    req: HttpRequest,
    data: web::Json<ImportShowRequest>,
) -> ApiResult<impl Responder> {
    let data = data.into_inner();
    let config: &Arc<Config> = req.app_data().unwrap();

    if data.episodes.is_empty() {
        return Err(ErrorBadRequest("show must have at least one episode"));
    }

    let show_path = Path::new(&config.libraries.tv_shows).join(&data.name);
    if show_path.exists() {
        return Err(ErrorBadRequest(format!("{:?} already exists", show_path)));
    }

    std::fs::create_dir(&show_path).map_err(ErrorInternalServerError)?;

    for episode in data.episodes {
        inner::import_episode(&show_path, episode).await?;
    }

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
pub struct ImportEpisodeRequest {
    source: ImportSource,
    season_number: u32,
    episode_number: u32,
}

pub async fn import_episode(
    req: HttpRequest,
    path: web::Path<(i64,)>,
    data: web::Json<ImportEpisodeRequest>,
) -> ApiResult<impl Responder> {
    let (show_id,) = path.into_inner();
    let data = data.into_inner();
    let db: &Db = req.app_data().unwrap();

    let mut conn = db.acquire().await.map_err(ErrorInternalServerError)?;
    let show_path: String = sqlx::query_scalar("SELECT path from tv_shows WHERE item_id = ?")
        .bind(show_id)
        .fetch_optional(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("show not found"))?;

    inner::import_episode(Path::new(&show_path), data).await
}

mod inner {
    use std::path::{Path, PathBuf};

    use actix_http::error::{ErrorBadRequest, ErrorInternalServerError};
    use actix_web::{HttpResponse, Responder};

    use crate::api::ApiResult;

    use super::{ImportEpisodeRequest, ImportSource};

    pub async fn import_episode(
        show_path: &Path,
        req: ImportEpisodeRequest,
    ) -> ApiResult<impl Responder> {
        let src_path = match req.source {
            ImportSource::Local { path } => path,
        };

        let src_path = PathBuf::from(src_path);
        let src_ext = src_path
            .extension()
            .ok_or_else(|| ErrorBadRequest("source file has no extension"))?;

        let dst_name = format!("S{:02}E{:02}", req.season_number, req.episode_number);
        let dst_path = Path::new(&show_path).join(dst_name).with_extension(src_ext);

        // Just move the file into the library and let the fs watcher
        // take care of the rest
        tracing::info!("moving {:?} to {:?}", src_path, dst_path);
        std::fs::rename(src_path, dst_path).map_err(ErrorInternalServerError)?;

        Ok(HttpResponse::Ok())
    }
}
