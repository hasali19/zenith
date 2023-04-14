use camino::Utf8Path;

use crate::{db, sql};

use super::MediaLibrary;

impl MediaLibrary {
    pub async fn import_subtitle(&self, path: &Utf8Path) -> eyre::Result<()> {
        let format = match path.extension() {
            Some("srt") => "srt",
            Some("vtt") => "webvtt",
            _ => return Ok(()),
        };

        let Some(meta) = self.parser().parse_subtitle_path(path) else {
            return Ok(());
        };

        let mut transaction = self.db.begin().await?;

        let sql = sql::select("video_files")
            .columns(&["id"])
            .condition("path_stem = ?")
            .to_sql();

        let search_path = path.parent().unwrap().join(meta.name);

        let video_id: Option<i64> = sqlx::query_scalar(&sql)
            .bind(search_path)
            .fetch_optional(&mut transaction)
            .await?;

        if let Some(video_id) = video_id {
            tracing::info!(%path, "importing subtitle");

            let subtitle = db::subtitles::NewSubtitle {
                video_id,
                stream_index: None,
                path: Some(path),
                title: Some(path.as_str()),
                language: meta.lang,
                format: Some(format),
                sdh: meta.sdh,
                forced: meta.forced,
            };

            tracing::info!(%video_id, %path, "adding subtitle");
            db::subtitles::insert(&mut transaction, &subtitle).await?;
        }

        transaction.commit().await?;

        Ok(())
    }

    pub async fn rescan_subtitle(&self, _path: &Utf8Path) -> eyre::Result<()> {
        Ok(())
    }

    pub async fn remove_subtitle(&self, path: &Utf8Path) -> eyre::Result<()> {
        tracing::info!(%path, "removing subtitle");

        let mut transaction = self.db.begin().await?;

        let res = sqlx::query("DELETE FROM subtitles WHERE path = ?")
            .bind(path)
            .execute(&mut transaction)
            .await?;

        if res.rows_affected() > 0 {
            tracing::info!(%path, "removed subtitle");
        }

        transaction.commit().await?;

        Ok(())
    }
}
