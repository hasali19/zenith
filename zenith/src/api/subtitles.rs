use std::str::FromStr;
use std::sync::Arc;

use atium::headers::ContentType;
use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request, Response};
use mime::Mime;

use crate::api::error::bad_request;
use crate::config::Config;
use crate::db::subtitles::SubtitlePath;
use crate::db::{self, Db};
use crate::subtitles;

use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router
        .route("/subtitles/:id")
        .get(get_subtitle)
        .delete(delete_subtitle);
}

#[endpoint]
async fn get_subtitle(req: &mut Request) -> eyre::Result<()> {
    let subtitle_id: i64 = req.param("id")?;

    let config: &Arc<Config> = req.ext().unwrap();
    let db: &Db = req.ext().unwrap();

    let mut conn = db.acquire().await?;

    let subtitle = db::subtitles::get_by_id(&mut conn, subtitle_id)
        .await?
        .or_not_found("subtitle not found")?;

    let path = db::videos::get_path(&mut conn, subtitle.video_id)
        .await?
        .or_not_found("video not found")?;

    match subtitle.path {
        db::subtitles::SubtitlePath::External(path) => {
            // Subtitle is an external file, return it directly
            req.respond_file(path.as_ref()).await?;
        }
        db::subtitles::SubtitlePath::Embedded(index) => {
            let cached_path = config
                .subtitles
                .path
                .join(subtitle.video_id.to_string())
                .join(format!("{}.extracted.vtt", index));

            if cached_path.is_file() {
                // Return directly if embedded subtitle has already been extracted
                tracing::info!("using cached subtitle");
                req.respond_file(cached_path).await?;
            } else {
                // Otherwise extract now
                // TODO: Cache the extracted subtitle?
                tracing::info!("extracting subtitle");

                let res = Response::ok()
                    .with_header(ContentType::from(Mime::from_str("text/vtt")?))
                    .with_body(subtitles::extract_embedded(config, &path, index).await?);

                req.set_res(res);
            }
        }
    }

    Ok(())
}

#[endpoint]
async fn delete_subtitle(req: &mut Request) -> eyre::Result<()> {
    let subtitle_id: i64 = req.param("id")?;
    let db: &Db = req.ext().unwrap();

    let mut conn = db.acquire().await?;

    let subtitle = db::subtitles::get_by_id(&mut conn, subtitle_id)
        .await?
        .or_not_found("subtitle not found")?;

    match subtitle.path {
        SubtitlePath::Embedded(_) => {
            return Err(eyre::eyre!(bad_request(
                "embedded subtitled cannot be deleted"
            )))
        }
        SubtitlePath::External(path) => {
            sqlx::query("DELETE FROM subtitles WHERE id = ?")
                .bind(subtitle_id)
                .execute(&mut conn)
                .await?;

            std::fs::remove_file(path.as_ref())?;
        }
    }

    req.ok();

    Ok(())
}
