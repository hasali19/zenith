use std::sync::Arc;

use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::Serialize;
use speq::Reflect;
use speq::axum::get;

use crate::config::Config;

use super::ApiResult;
use super::ext::OptionExt;

#[derive(Serialize, Reflect)]
struct CastConfig {
    app_id: String,
}

#[get("/cast/config")]
pub async fn get_cast_config(config: Extension<Arc<Config>>) -> ApiResult<impl IntoResponse> {
    let app_id = config.cast.app_id.clone();
    Ok(Json(CastConfig {
        app_id: app_id.or_not_found("cast application is not configured")?,
    }))
}
