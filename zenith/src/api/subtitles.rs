use std::process::Stdio;
use std::str::FromStr;
use std::sync::Arc;

use atium::headers::ContentType;
use atium::respond::RespondRequestExt;
use atium::router::{Router, RouterRequestExt};
use atium::{endpoint, Request, Response};
use mime::Mime;
use tokio::process::Command;

use crate::config::Config;
use crate::db::{self, Db};
use crate::ext::CommandExt;

use super::ext::OptionExt;

pub fn routes(router: &mut Router) {
    router.route("/subtitles/:id").get(get_subtitle);
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
                    .with_body(extract_embedded_subtitle(config, &path, index).await?);

                req.set_res(res);
            }
        }
    }

    Ok(())
}

async fn extract_embedded_subtitle(
    config: &Config,
    path: &str,
    index: u32,
) -> std::io::Result<Vec<u8>> {
    Command::new(&config.transcoding.ffmpeg_path)
        .arg_pair("-i", &path)
        .arg_pair("-map", format!("0:{}", index))
        .arg_pair("-c:s", "webvtt")
        .arg_pair("-f", "webvtt")
        .arg("pipe:1")
        .stdout(Stdio::piped())
        .output()
        .await
        .map(|output| output.stdout)
}
