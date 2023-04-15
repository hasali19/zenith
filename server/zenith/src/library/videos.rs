use camino::Utf8Path;
use eyre::eyre;
use sqlx::SqliteConnection;

use crate::db::streams::{NewAudioStream, NewVideoStream};
use crate::db::subtitles::NewSubtitle;
use crate::db::videos::UpdateVideo;
use crate::video_prober::VideoInfo;
use crate::{db, sql};

use super::scanner::VideoFileType;
use super::{LibraryEvent, MediaLibrary};

impl MediaLibrary {
    pub async fn import_video(
        &self,
        video_type: VideoFileType,
        path: &Utf8Path,
    ) -> eyre::Result<()> {
        match video_type {
            VideoFileType::Movie => self.import_movie(path).await,
            VideoFileType::Episode => self.import_episode(path).await,
        }
    }

    pub async fn rescan_video(&self, path: &Utf8Path) -> eyre::Result<()> {
        tracing::debug!(%path, "rescanning video file");

        let mut transaction = self.db.begin().await?;

        let info = self.video_prober.probe(path).await?;
        let video_id = sqlx::query_scalar("SELECT id FROM video_files WHERE path = ?")
            .bind(path)
            .fetch_one(&mut transaction)
            .await?;

        update_video_info(&mut transaction, video_id, &info).await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn remove_video(&self, path: &Utf8Path) -> eyre::Result<()> {
        tracing::info!(%path, "removing video");
        let mut transaction = self.db.begin().await?;
        db::video_files::remove_by_path(&mut transaction, path).await?;
        transaction.commit().await?;
        Ok(())
    }

    pub(super) async fn create_video_file(
        &self,
        path: &Utf8Path,
        media_id: i64,
    ) -> eyre::Result<()> {
        let info = self.video_prober.probe(path).await?;
        let duration: f64 = info.format.duration.parse()?;

        let mut transaction = self.db.begin().await?;

        let sql = sql::insert("video_files")
            .columns(&["item_id", "path", "duration", "scanned_at"])
            .values(&["?", "?", "?", "strftime('%s')"])
            .returning(&["id"])
            .to_sql();

        let video_id = sqlx::query_scalar(&sql)
            .bind(media_id)
            .bind(path)
            .bind(duration)
            .fetch_one(&mut transaction)
            .await?;

        update_video_info(&mut transaction, video_id, &info).await?;

        transaction.commit().await?;

        let _ = self.notifier.send(LibraryEvent::VideoAdded(video_id));

        Ok(())
    }
}

async fn update_video_info(
    conn: &mut SqliteConnection,
    id: i64,
    info: &VideoInfo,
) -> eyre::Result<()> {
    let data = UpdateVideo {
        path: None,
        duration: Some(info.format.duration.parse()?),
        format_name: Some(Some(info.format.format_name.as_str())),
        set_scanned_at: true,
    };

    db::videos::update(&mut *conn, id, data).await?;

    for stream in &info.streams {
        let Some(codec_name) = stream.codec_name.as_deref() else {
            tracing::debug!(stream.index, stream.codec_type, "stream is missing codec name");
            continue;
        };

        match stream.codec_type.as_str() {
            "video" => {
                let (width, height) =
                    if let (Some(width), Some(height)) = (stream.width, stream.height) {
                        (width as u32, height as u32)
                    } else {
                        return Err(eyre!("missing width and height for video stream"));
                    };

                let stream = NewVideoStream {
                    video_id: id,
                    index: stream.index,
                    codec_name,
                    width,
                    height,
                };

                db::streams::insert_video_stream(&mut *conn, &stream).await?;
            }
            "audio" => {
                let language = stream
                    .tags
                    .as_ref()
                    .and_then(|tags| tags.language.as_deref());

                let stream = NewAudioStream {
                    video_id: id,
                    index: stream.index,
                    codec_name,
                    language,
                    channels: stream.channels,
                    channel_layout: stream.channel_layout.as_deref(),
                };

                db::streams::insert_audio_stream(&mut *conn, &stream).await?;
            }
            "subtitle" => {
                let title = stream.tags.as_ref().and_then(|tags| tags.title.as_deref());
                let language = stream
                    .tags
                    .as_ref()
                    .and_then(|tags| tags.language.as_deref());

                let get_disposition_bool = |name| {
                    stream
                        .properties
                        .get("disposition")
                        .and_then(|v| v.as_object())
                        .and_then(|v| v.get(name))
                        .and_then(|v| v.as_i64())
                        .map(|v| v == 1)
                        .unwrap_or(false)
                };

                let subtitle = NewSubtitle {
                    video_id: id,
                    stream_index: Some(stream.index),
                    path: None,
                    title,
                    language,
                    format: Some(codec_name),
                    sdh: get_disposition_bool("hearing_impaired"),
                    forced: get_disposition_bool("forced"),
                };

                db::subtitles::insert(&mut *conn, &subtitle).await?;
            }
            _ => {}
        }
    }

    // Remove streams which no longer exist in the file
    db::streams::remove_except(&mut *conn, id, info.streams.iter().map(|s| s.index)).await?;

    Ok(())
}
