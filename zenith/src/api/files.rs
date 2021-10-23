use std::sync::Arc;

use actix_web::web::{Json, Query};
use actix_web::{get, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::api::ApiResult;
use crate::config::Config;
use crate::Ext;

use super::error::bad_request;

#[derive(Deserialize)]
struct GetFilesQuery {
    path: Option<String>,
}

#[get("/files")]
async fn get_files(
    query: Query<GetFilesQuery>,
    config: Ext<Arc<Config>>,
) -> ApiResult<impl Responder> {
    let path = query
        .path
        .as_deref()
        .or_else(|| config.import.path.as_deref());

    let path = match path {
        Some(path) => path,
        None => {
            return Err(bad_request(
                "No path specified, and no default import path has been configured",
            )
            .into());
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

    Ok(Json(files))
}
