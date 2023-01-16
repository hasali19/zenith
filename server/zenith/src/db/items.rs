use std::fmt::Write;

use itertools::Itertools;
use serde::Serialize;
use speq::Reflect;
use sqlx::sqlite::{SqliteArguments, SqliteRow};
use sqlx::{Acquire, Arguments, FromRow, Row, SqliteConnection};

use crate::sql::{self, Join};

use super::media::{MediaImage, MediaItemType, MetadataProvider};
use super::streams::StreamType;

#[derive(Serialize, Reflect)]
pub struct ExternalIds {
    pub tmdb: Option<i32>,
}

#[derive(Debug)]
pub struct MediaItem {
    pub id: i64,
    pub kind: MediaItemType,
    pub name: String,
    pub overview: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub poster: Option<String>,
    pub backdrop: Option<String>,
    pub thumbnail: Option<String>,
    pub age_rating: Option<String>,
    pub genres: Vec<String>,
    pub tmdb_id: Option<i32>,
    pub imdb_id: Option<String>,
    pub parent: Option<Parent>,
    pub grandparent: Option<Parent>,
    pub video_file: Option<VideoFile>,
    pub metadata_provider: Option<String>,
    pub metadata_provider_key: Option<String>,
}

#[derive(Debug)]
pub struct Parent {
    pub id: i64,
    pub index: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct VideoFile {
    pub item_id: i64,
    pub path: String,
    pub duration: Option<f64>,
    pub format_name: Option<String>,
    pub streams: Vec<Stream>,
    pub subtitles: Vec<Subtitle>,
}

#[derive(Debug)]
pub struct Stream {
    pub id: i64,
    pub video_id: i64,
    pub index: u32,
    pub props: StreamProps,
    pub codec_name: String,
}

#[derive(Debug)]
pub enum StreamProps {
    Video(VideoStreamProps),
    Audio(AudioStreamProps),
}

#[derive(Debug)]
pub struct VideoStreamProps {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct AudioStreamProps {
    pub language: Option<String>,
}

#[derive(Debug)]
pub struct Subtitle {
    pub id: i64,
    pub video_id: i64,
    pub stream_index: Option<i64>,
    pub path: Option<String>,
    pub title: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug)]
pub struct VideoUserData {
    pub item_id: i64,
    pub position: f64,
    pub is_watched: bool,
    pub last_watched_at: Option<i64>,
}

#[derive(Debug)]
pub struct CollectionUserData {
    pub item_id: i64,
    pub unwatched: u32,
}

impl<'r> FromRow<'r, SqliteRow> for MediaItem {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        fn get_parent(
            row: &SqliteRow,
            id_col: &str,
            index_col: &str,
            name_col: &str,
        ) -> sqlx::Result<Option<Parent>> {
            let id = row.try_get(id_col)?;
            let index = row.try_get(index_col)?;
            let name = row.try_get(name_col)?;
            if let (Some(id), Some(index), Some(name)) = (id, index, name) {
                Ok(Some(Parent { id, index, name }))
            } else {
                Ok(None)
            }
        }

        let video_file = if let Some(path) = row.try_get("path")? {
            Some(VideoFile {
                path,
                item_id: row.try_get("id")?,
                duration: row.try_get("duration")?,
                format_name: row.try_get("format_name")?,
                streams: vec![],
                subtitles: vec![],
            })
        } else {
            None
        };

        Ok(MediaItem {
            id: row.try_get("id")?,
            kind: row.try_get("item_type")?,
            name: row.try_get("name")?,
            overview: row.try_get("overview")?,
            start_date: row.try_get("start_date")?,
            end_date: row.try_get("end_date")?,
            poster: row.try_get("poster")?,
            backdrop: row.try_get("backdrop")?,
            thumbnail: row.try_get("thumbnail")?,
            age_rating: row.try_get("age_rating")?,
            genres: vec![],
            tmdb_id: row.try_get("tmdb_id")?,
            imdb_id: row.try_get("imdb_id")?,
            parent: get_parent(row, "parent_id", "parent_index", "parent_name")?,
            grandparent: get_parent(
                row,
                "grandparent_id",
                "grandparent_index",
                "grandparent_name",
            )?,
            video_file,
            metadata_provider: row.try_get("metadata_provider")?,
            metadata_provider_key: row.try_get("metadata_provider_key")?,
        })
    }
}

impl<'r> FromRow<'r, SqliteRow> for Stream {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let stream_type: StreamType = row.try_get("stream_type")?;

        Ok(Stream {
            id: row.try_get("id")?,
            video_id: row.try_get("video_id")?,
            index: row.try_get("stream_index")?,
            codec_name: row.try_get("codec_name")?,
            props: match stream_type {
                StreamType::Audio => StreamProps::Audio(AudioStreamProps {
                    language: row.try_get("a_language")?,
                }),
                StreamType::Video => StreamProps::Video(VideoStreamProps {
                    width: row.try_get("v_width")?,
                    height: row.try_get("v_height")?,
                }),
            },
        })
    }
}

impl<'r> FromRow<'r, SqliteRow> for Subtitle {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Subtitle {
            id: row.try_get("id")?,
            video_id: row.try_get("video_id")?,
            stream_index: row.try_get("stream_index")?,
            path: row.try_get("path")?,
            title: row.try_get("title")?,
            language: row.try_get("language")?,
        })
    }
}

impl<'r> FromRow<'r, SqliteRow> for VideoUserData {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(VideoUserData {
            item_id: row.try_get("item_id")?,
            position: row.try_get("position")?,
            is_watched: row.try_get("is_watched")?,
            last_watched_at: row.try_get("last_watched_at")?,
        })
    }
}

impl<'r> FromRow<'r, SqliteRow> for CollectionUserData {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(CollectionUserData {
            item_id: row.try_get("item_id")?,
            unwatched: row.try_get("unwatched")?,
        })
    }
}

const ITEM_COLUMNS: &[&str] = &[
    "m.id",
    "m.item_type",
    "m.name",
    "m.overview",
    "m.start_date",
    "m.end_date",
    "m.metadata_provider",
    "m.metadata_provider_key",
    "COALESCE(m.poster, parent.poster, grandparent.poster) AS poster",
    "COALESCE(m.backdrop, parent.backdrop, grandparent.backdrop) AS backdrop",
    "COALESCE(m.thumbnail, m.backdrop, parent.backdrop, grandparent.backdrop) AS thumbnail",
    "m.age_rating",
    "m.tmdb_id",
    "m.imdb_id",
    "m.parent_id",
    "m.parent_index",
    "parent.name AS parent_name",
    "m.grandparent_id",
    "m.grandparent_index",
    "grandparent.name AS grandparent_name",
    "path",
    "duration",
    "format_name",
];

const STREAM_COLUMNS: &[&str] = &[
    "id",
    "video_id",
    "stream_index",
    "stream_type",
    "codec_name",
    "v_width",
    "v_height",
    "a_language",
];

const SUBTITLE_COLUMNS: &[&str] = &[
    "id",
    "video_id",
    "stream_index",
    "path",
    "title",
    "language",
];

#[derive(Debug, Default)]
pub struct Query<'a> {
    pub ids: Option<&'a [i64]>,
    pub item_type: Option<MediaItemType>,
    pub parent_id: Option<i64>,
    pub grandparent_id: Option<i64>,
    pub collection_id: Option<i64>,
    pub is_watched: Option<bool>,
    pub sort_by: &'a [SortField],
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug)]
pub enum SortField {
    Name,
    ParentIndex,
    GrandparentIndex,
    CollectionIndex,
}

pub async fn query(conn: &mut SqliteConnection, query: Query<'_>) -> eyre::Result<Vec<MediaItem>> {
    let mut args = SqliteArguments::default();
    let mut conditions = vec![];
    let mut joins = vec![
        Join::left("video_files AS v ON v.item_id = m.id"),
        Join::left("user_item_data AS u ON u.item_id = m.id"),
        Join::left("media_items AS parent ON parent.id = m.parent_id"),
        Join::left("media_items AS grandparent ON grandparent.id = m.grandparent_id"),
    ];

    if let Some(ids) = query.ids {
        conditions.push(format!("m.id IN ({})", sql::Placeholders(ids.len())));
        for index in ids {
            args.add(index);
        }
    }

    if let Some(item_type) = query.item_type {
        conditions.push("m.item_type = ?".to_owned());
        args.add(item_type);
    }

    if let Some(id) = query.parent_id {
        conditions.push("m.parent_id = ?".to_owned());
        args.add(id);
    }

    if let Some(id) = query.grandparent_id {
        conditions.push("m.grandparent_id = ?".to_owned());
        args.add(id);
    }

    if let Some(id) = query.collection_id {
        joins.push(Join::left(
            "collections_media_items AS c ON c.item_id = m.id",
        ));
        conditions.push("c.collection_id = ?".to_owned());
        args.add(id);
    }

    if let Some(is_watched) = query.is_watched {
        conditions.push("COALESCE(u.is_watched, 0) = ?".to_owned());
        args.add(is_watched);
    }

    let order_by = query
        .sort_by
        .iter()
        .map(|field| match field {
            SortField::Name => "m.name",
            SortField::ParentIndex => "m.parent_index",
            SortField::GrandparentIndex => "m.grandparent_index",
            SortField::CollectionIndex => "c.idx",
        })
        .collect_vec();

    let sql = sql::select("media_items AS m")
        .columns(ITEM_COLUMNS)
        .joins(&joins)
        .condition(&conditions.join(" AND "))
        .order_by(&order_by)
        .limit(query.limit)
        .to_sql();

    let mut items: Vec<MediaItem> = sqlx::query_as_with(&sql, args.clone())
        .fetch_all(&mut *conn)
        .await?;

    let ids = items.iter().map(|item| item.id).collect_vec();

    let sql = sql::select("video_file_streams")
        .columns(STREAM_COLUMNS)
        .condition(&format!("video_id IN ({})", sql::Placeholders(ids.len())))
        .to_sql();

    let mut streams = sqlx::query_as_with(&sql, args.clone())
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .into_group_map_by(|stream: &Stream| stream.video_id);

    let sql = sql::select("subtitles")
        .columns(SUBTITLE_COLUMNS)
        .condition(&format!("video_id IN ({})", sql::Placeholders(ids.len())))
        .to_sql();

    let mut subtitles = sqlx::query_as_with(&sql, args.clone())
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .into_group_map_by(|sub: &Subtitle| sub.video_id);

    let sql = sql::select("genres AS g")
        .columns(&["mg.item_id, g.name"])
        .joins(&[Join::inner(
            "media_items_genres AS mg ON mg.genre_id = g.id",
        )])
        .condition(&format!("mg.item_id IN ({})", sql::Placeholders(ids.len())))
        .to_sql();

    let mut genres = sqlx::query_as_with::<_, (i64, String), _>(&sql, args)
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .into_grouping_map_by(|(item_id, _)| *item_id)
        .aggregate(|acc: Option<Vec<String>>, _, (_, v)| {
            if let Some(mut acc) = acc {
                acc.push(v);
                Some(acc)
            } else {
                Some(vec![v])
            }
        });

    for item in &mut items {
        item.genres = genres.remove(&item.id).unwrap_or_default();

        if let Some(video) = &mut item.video_file {
            video.streams = streams.remove(&video.item_id).unwrap_or_default();
            video.subtitles = subtitles.remove(&video.item_id).unwrap_or_default();
        }
    }

    Ok(items)
}

pub async fn get(conn: &mut SqliteConnection, id: i64) -> eyre::Result<Option<MediaItem>> {
    let ids = [id];
    let query = Query {
        ids: Some(&ids),
        ..Default::default()
    };

    self::query(conn, query)
        .await
        .map(|res| res.into_iter().next())
}

pub async fn get_continue_watching(
    conn: &mut SqliteConnection,
    limit: Option<u32>,
) -> eyre::Result<Vec<i64>> {
    // This beautiful query does two things:
    // - for movies, we grab ids of all movies where the user position is within the "currently watching" range
    // - for each show, we grab the last episode that was watched; if that episode was finished, then we instead
    //   get the next episode if it exists
    let mut sql = "
        SELECT id, last_watched_at FROM (
            SELECT m.id AS id, u.last_watched_at AS last_watched_at FROM movies AS m
            JOIN video_files AS v ON v.item_id = m.id
            LEFT JOIN user_item_data AS u ON m.id = u.item_id
            WHERE u.position > (0.05 * v.duration) AND u.position < (0.9 * v.duration) AND u.last_watched_at IS NOT NULL
        )
        UNION
        SELECT id, last_watched_at FROM (
            SELECT IIF(
                u.position < (0.9 * v.duration),
                -- return current episode if the position is below 'completed' threshold
                e.id,
                -- otherwise find the next episode
                (
                    SELECT e1.id FROM episodes AS e1
                    JOIN seasons AS season1 ON season1.id = e1.season_id
                    JOIN shows AS show1 ON show1.id = season1.show_id
                    WHERE show1.id = show.id
                        AND (season1.season_no > season.season_no
                            OR (season1.season_no = season.season_no AND e1.episode_no > e.episode_no))
                    ORDER BY season1.season_no, e1.episode_no
                    LIMIT 1
                )
            ) AS id, MAX(last_watched_at) AS last_watched_at FROM episodes AS e
            JOIN seasons AS season ON season.id = e.season_id
            JOIN shows AS show ON show.id = season.show_id
            JOIN video_files AS v ON v.item_id = e.id
            LEFT JOIN user_item_data AS u ON e.id = u.item_id
            WHERE u.position > (0.05 * v.duration) AND u.last_watched_at IS NOT NULL
            GROUP BY show.id
        )
        WHERE id IS NOT NULL
        ORDER BY last_watched_at DESC
    ".to_owned();

    if let Some(limit) = limit {
        write!(sql, "LIMIT {}", limit).unwrap();
    }

    Ok(sqlx::query_scalar(&sql).fetch_all(&mut *conn).await?)
}

pub async fn get_recently_added_movies(conn: &mut SqliteConnection) -> eyre::Result<Vec<i64>> {
    let sql = sql::select("movies AS m")
        .columns(&["m.id"])
        .joins(&[Join::left("user_item_data AS u ON u.item_id = m.id")])
        .condition("COALESCE(u.is_watched, 0) = 0")
        .order_by(&["added_at DESC", "name"])
        .limit(30)
        .to_sql();

    Ok(sqlx::query_scalar(&sql).fetch_all(&mut *conn).await?)
}

pub async fn get_recently_updated_shows(conn: &mut SqliteConnection) -> eyre::Result<Vec<i64>> {
    let condition = "
        (
            SELECT COUNT(*)
            FROM episodes AS episode
            LEFT JOIN user_item_data AS u ON u.item_id = episode.id
            WHERE episode.show_id = s.id AND COALESCE(u.is_watched, 0) = 0
        ) > 0
    ";

    // Get shows sorted by the added_at of their most recently added episode
    // (i.e. shows that have had an episode added recently will appear higher up)
    let sql = sql::select("shows AS s")
        .columns(&["s.id"])
        .joins(&[Join::inner("episodes AS e").on("e.show_id = s.id")])
        .condition(condition)
        .group_by("s.id")
        .order_by(&["MAX(e.added_at) DESC", "s.name"])
        .limit(30)
        .to_sql();

    Ok(sqlx::query_scalar(&sql).fetch_all(conn).await?)
}

pub async fn get_user_data_for_video(
    conn: &mut SqliteConnection,
    id: i64,
) -> eyre::Result<Option<VideoUserData>> {
    Ok(get_user_data_for_videos(conn, &[id])
        .await?
        .into_iter()
        .next())
}

pub async fn get_user_data_for_videos(
    conn: &mut SqliteConnection,
    ids: &[i64],
) -> eyre::Result<Vec<VideoUserData>> {
    let mut args = SqliteArguments::default();
    for index in ids {
        args.add(index);
    }

    let sql = sql::select("video_files AS v")
        .joins(&[Join::left("user_item_data AS u ON u.item_id = v.item_id")])
        .columns(&[
            "v.item_id",
            "COALESCE(position, 0.0) AS position",
            "COALESCE(is_watched, 0) AS is_watched",
            "last_watched_at",
        ])
        .condition(&format!("v.item_id IN ({})", sql::Placeholders(ids.len())))
        .to_sql();

    Ok(sqlx::query_as_with(&sql, args).fetch_all(conn).await?)
}

pub async fn get_user_data_for_collections(
    conn: &mut SqliteConnection,
    ids: &[i64],
) -> eyre::Result<Vec<CollectionUserData>> {
    let mut args = SqliteArguments::default();
    for index in ids {
        args.add(index);
    }

    let mut placeholders = String::new();
    for i in 1..=ids.len() {
        write!(placeholders, "?{i}")?;
        if i != ids.len() {
            write!(placeholders, ",")?;
        }
    }

    #[rustfmt::skip]
    let sql = format!("
        SELECT v.parent_id AS item_id, COUNT(*) AS unwatched
        FROM media_items AS v
        LEFT JOIN user_item_data AS u ON u.item_id = v.id
        WHERE v.id IN (SELECT item_id FROM video_files)
            AND v.parent_id IN ({placeholders})
            AND COALESCE(u.is_watched, 0) = 0
        GROUP BY v.parent_id
        UNION
        SELECT v.grandparent_id AS item_id, COUNT(*) AS unwatched
        FROM media_items AS v
        LEFT JOIN user_item_data AS u ON u.item_id = v.id
        WHERE v.id IN (SELECT item_id FROM video_files)
            AND v.grandparent_id IN ({placeholders})
            AND COALESCE(u.is_watched, 0) = 0
        GROUP BY v.grandparent_id
    ");

    Ok(sqlx::query_as_with(&sql, args).fetch_all(conn).await?)
}

#[derive(Default)]
pub struct UpdateMetadata<'a> {
    pub name: Option<&'a str>,
    pub overview: Option<Option<&'a str>>,
    pub start_date: Option<Option<i64>>,
    pub end_date: Option<Option<i64>>,
    pub poster: Option<Option<MediaImage<'a>>>,
    pub backdrop: Option<Option<MediaImage<'a>>>,
    pub thumbnail: Option<Option<MediaImage<'a>>>,
    pub age_rating: Option<Option<&'a str>>,
    pub genres: Option<&'a [&'a str]>,
    pub tmdb_id: Option<Option<i32>>,
    pub imdb_id: Option<Option<&'a str>>,
    pub metadata_provider: Option<Option<MetadataProvider>>,
    pub metadata_provider_key: Option<Option<&'a str>>,
}

pub async fn update_metadata(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateMetadata<'_>,
) -> eyre::Result<()> {
    let mut columns = vec![];
    let mut values = vec![];
    let mut args = SqliteArguments::default();

    macro_rules! collect {
        ($field:ident) => {
            if let Some($field) = data.$field {
                columns.push(stringify!($field));
                values.push("?");
                args.add($field);
            }
        };
        ($field:ident, $($fields:ident),+) => {
            collect!($field);
            $(collect!($fields));+
        }
    }

    collect!(
        name,
        overview,
        start_date,
        end_date,
        age_rating,
        tmdb_id,
        imdb_id,
        metadata_provider,
        metadata_provider_key
    );

    for (column, img) in [
        ("poster", data.poster),
        ("backdrop", data.backdrop),
        ("thumbnail", data.thumbnail),
    ] {
        if let Some(img) = img {
            columns.push(column);
            values.push("?");
            args.add(img.map(|img| img.to_string()));
        }
    }

    columns.push("metadata_updated_at");
    values.push("(strftime('%s', 'now'))");

    args.add(id);

    let sql = sql::update("media_items")
        .columns(&columns)
        .values(&values)
        .condition("id = ?")
        .to_sql();

    let mut tx = conn.begin().await?;

    sqlx::query_with(&sql, args).execute(&mut tx).await?;

    if let Some(genres) = data.genres {
        let genre_ids = if !genres.is_empty() {
            let mut args = SqliteArguments::default();
            let mut placeholders = String::new();

            for (i, &genre) in genres.iter().enumerate() {
                args.add(genre);
                placeholders += "(?)";
                if i < genres.len() - 1 {
                    placeholders += ",";
                }
            }

            #[rustfmt::skip]
            let sql = format!("
                INSERT INTO genres (name) VALUES {placeholders}
                ON CONFLICT DO UPDATE SET name = excluded.name
                RETURNING (id)
            ");

            let genre_ids: Vec<i64> = sqlx::query_scalar_with(&sql, args)
                .fetch_all(&mut tx)
                .await?;

            genre_ids
        } else {
            vec![]
        };

        let mut args = SqliteArguments::default();
        let mut placeholders = String::new();

        for (i, genre_id) in genre_ids.iter().enumerate() {
            args.add(id);
            args.add(genre_id);
            placeholders += "(?, ?)";
            if i < genre_ids.len() - 1 {
                placeholders += ",";
            }
        }

        #[rustfmt::skip]
        let sql = format!("
            INSERT OR IGNORE INTO media_items_genres (item_id, genre_id)
            VALUES {placeholders}
        ");

        sqlx::query_with(&sql, args).execute(&mut tx).await?;

        let placeholders = sql::Placeholders(genre_ids.len());
        let mut args = SqliteArguments::default();
        for genre_id in &genre_ids {
            args.add(genre_id);
        }

        #[rustfmt::skip]
        let sql = format!("
            DELETE FROM media_items_genres
            WHERE item_id = ? AND genre_id NOT IN ({placeholders})
        ");

        sqlx::query_with(&sql, args).execute(&mut tx).await?;
    }

    tx.commit().await?;

    Ok(())
}
