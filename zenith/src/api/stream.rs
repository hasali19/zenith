use actix_files::NamedFile;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, Responder};

use crate::db::Db;

use super::{ApiError, ApiResult};

pub fn service(path: &str) -> impl HttpServiceFactory {
    web::scope(path).route("/{id}", web::get().to(get_stream))
}

async fn get_stream(path: web::Path<(i64,)>, db: Db) -> ApiResult<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = db.acquire().await?;

    let path: Option<(String,)> = sqlx::query_as("SELECT path FROM media_files WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?;

    let path = match path {
        Some((path,)) => path,
        None => return Err(ApiError::NotFound),
    };

    Ok(NamedFile::open(path))
}
