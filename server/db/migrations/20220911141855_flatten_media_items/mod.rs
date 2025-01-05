// hash:3bcb3d0c064e6193f7c8d41b3ea24a51400357add21ee6d01e09eae4639bb14f

use std::path::Path;

use sqlx::{QueryBuilder, Sqlite};

use crate::WriteConnection;

pub async fn execute(conn: &mut WriteConnection) -> eyre::Result<()> {
    // Execute the main migration script.
    sqlx::query(include_str!("script.sql"))
        .execute(&mut *conn)
        .await?;

    // We need to add all movie directory paths to the new `indexed_paths` table. This requires more
    // complex logic than can be comfortably written in sql.

    // Get ids and video paths for each movie item.
    let sql = "
        select media_items.id, video_files.path from media_items
        join video_files on media_items.id = video_files.item_id
        where item_type = 1";

    let movies: Vec<(i64, String)> = sqlx::query_as(sql).fetch_all(&mut *conn).await?;

    for chunk in movies.chunks(50) {
        let mut query = QueryBuilder::<Sqlite>::new("insert into indexed_paths (item_id, path) ");

        query.push_values(chunk, |mut b, (id, path)| {
            b.push_bind(id)
                // Add the parent directory of the movie file to the index
                .push_bind(Path::new(path).parent().unwrap().to_str().unwrap());
        });

        query.build().execute(&mut *conn).await?;
    }

    Ok(())
}
