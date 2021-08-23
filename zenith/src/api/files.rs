use std::sync::Arc;

use actix_web::error::ErrorBadRequest;
use actix_web::web::{self, Query, ServiceConfig};
use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::config::Config;

use super::ApiResult;

pub fn configure(config: &mut ServiceConfig) {
    config.route("/files", web::get().to(get_files));
}

#[derive(Deserialize)]
struct GetFilesQuery {
    path: Option<String>,
}

async fn get_files(req: HttpRequest, Query(query): Query<GetFilesQuery>) -> ApiResult {
    let config: &Arc<Config> = req.app_data().unwrap();

    let path = query.path.as_deref().or(config.import.path.as_deref());
    let path = match path {
        Some(path) => path,
        None => {
            return Err(ErrorBadRequest(
                "No path specified, and no default import path has been configured",
            ));
        }
    };

    let files = std::fs::read_dir(path)?
        .filter_map(|f| f.ok())
        .filter_map(|f| {
            let name = f.file_name();
            let path = f.path().canonicalize().ok()?;
            let file_type = f.file_type().ok()?;

            let data = json!({
                "name": name.to_str()?,
                "path": &path,
                "type": if file_type.is_file() {
                    "file"
                } else if file_type.is_dir() {
                    "dir"
                } else {
                    return None;
                },
            });

            Some(data)
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(&files))
}
