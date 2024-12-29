use itertools::Itertools;
use sqlx::SqliteConnection;

use crate::images::{self, ImageSourceType, ImageType};

pub async fn execute(conn: &mut SqliteConnection) -> eyre::Result<()> {
    // Execute the main migration script.
    sqlx::query(include_str!("add_tables.sql"))
        .execute(&mut *conn)
        .await?;

    type ItemRow = (i64, Option<String>, Option<String>, Option<String>);

    let rows: Vec<ItemRow> =
        sqlx::query_as("SELECT id, poster, backdrop, thumbnail FROM media_items")
            .fetch_all(&mut *conn)
            .await?;

    for (item_id, poster, backdrop, thumbnail) in rows {
        for (image, image_type) in [
            (poster, ImageType::Poster),
            (backdrop, ImageType::Backdrop),
            (thumbnail, ImageType::Thumbnail),
        ] {
            let Some(image) = image else { continue };

            let (_img_type, src_type, src) = image.splitn(3, '|').collect_tuple().unwrap();

            assert_eq!(src_type, "2" /* MediaImageSrcType::Tmdb */);

            let id =
                images::get_or_create(&mut *conn, image_type, ImageSourceType::Tmdb, src).await?;

            let column = match image_type {
                ImageType::Poster => "poster",
                ImageType::Backdrop => "backdrop",
                ImageType::Thumbnail => "thumbnail",
                ImageType::Profile => "profile",
            };

            let sql = format!("UPDATE media_items SET {column} = ? WHERE id = ?");

            sqlx::query(&sql)
                .bind(id)
                .bind(item_id)
                .execute(&mut *conn)
                .await?;
        }
    }

    type PeopleRow = (i64, Option<String>);

    let rows: Vec<PeopleRow> = sqlx::query_as("SELECT id, profile FROM people")
        .fetch_all(&mut *conn)
        .await?;

    for (people_id, image) in rows {
        let Some(image) = image else { continue };

        let (_img_type, src_type, src) = image.splitn(3, '|').collect_tuple().unwrap();

        assert_eq!(src_type, "2" /* MediaImageSrcType::Tmdb */);

        let id = images::get_or_create(&mut *conn, ImageType::Profile, ImageSourceType::Tmdb, src)
            .await?;

        sqlx::query("UPDATE people SET profile = ? WHERE id = ?")
            .bind(id)
            .bind(people_id)
            .execute(&mut *conn)
            .await?;
    }

    sqlx::query(include_str!("update_foreign_keys.sql"))
        .execute(&mut *conn)
        .await?;

    Ok(())
}
