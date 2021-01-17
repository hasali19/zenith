pub mod movies;
pub mod tv_shows;

use std::sync::Arc;

use crate::config::Config;
use crate::db::Db;
use crate::ffmpeg::Ffprobe;
use crate::tmdb::TmdbClient;

pub async fn full_library_sync(
    db: &Db,
    tmdb: &TmdbClient,
    config: Arc<Config>,
) -> eyre::Result<()> {
    let ffprobe = Ffprobe::new(config.ffprobe_path());
    let mut conn = db.acquire().await?;

    movies::sync_movies(&mut conn, &tmdb, &ffprobe, &config.movie_path).await?;
    tv_shows::sync_tv_shows(&mut conn, &tmdb, &ffprobe, &config.tv_show_path).await?;

    Ok(())
}
