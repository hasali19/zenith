use std::sync::Arc;

use atium::query::QueryRequestExt;
use atium::responder::Json;
use atium::router::Router;
use atium::{endpoint, Request, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::config::Config;

use super::error::bad_request;

pub fn routes(router: &mut Router) {
    router.route("/files").get(get_files);
}

#[derive(Deserialize)]
struct GetFilesQuery {
    path: Option<String>,
}

#[endpoint]
async fn get_files(req: &mut Request) -> eyre::Result<impl Responder> {
    let query: GetFilesQuery = req.query()?;
    let config: &Arc<Config> = req.ext().unwrap();

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
