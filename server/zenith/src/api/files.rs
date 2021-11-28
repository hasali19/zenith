use std::sync::Arc;

use axum::extract::{Extension, Query};
use axum::response::IntoResponse;
use axum::Json;
use axum_codegen::get;
use serde::Deserialize;
use serde_json::json;

use crate::api::ApiResult;
use crate::config::Config;

use super::error::bad_request;

#[derive(Deserialize)]
struct GetFilesQuery {
    path: Option<String>,
}

#[get("/files")]
async fn get_files(
    query: Query<GetFilesQuery>,
    config: Extension<Arc<Config>>,
) -> ApiResult<impl IntoResponse> {
    let path = query
        .path
        .as_deref()
        .or_else(|| config.import.path.as_deref());

    let path = match path {
        Some(path) => path,
        None => {
            return Err(bad_request(
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

    Ok(Json(files))
}
