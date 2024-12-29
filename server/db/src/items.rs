use std::fmt::Write;

use camino::Utf8PathBuf;
use eyre::{eyre, Context};
use itertools::Itertools;
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, FromRow, Row, SqliteConnection};

use crate::sql::{self, Join};
use crate::utils::arguments::QueryArguments;

use super::media::{MediaItemType, MetadataProvider};
use super::streams::StreamType;

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
    pub trailer: Option<String>,
    pub tmdb_id: Option<i32>,
    pub imdb_id: Option<String>,
    pub cast: Vec<CastMember>,
    pub director: Option<String>,
    pub parent: Option<Parent>,
    pub grandparent: Option<Parent>,
    pub video_file: Option<VideoFile>,
    pub metadata_provider: Option<String>,
    pub metadata_provider_key: Option<String>,
}

#[derive(Debug)]
pub struct CastMember {
    pub name: String,
    pub character: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug)]
pub struct Parent {
    pub id: i64,
    pub index: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct VideoFile {
    pub id: i64,
    pub item_id: i64,
    pub path: Utf8PathBuf,
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
    pub crop_x1: Option<u32>,
    pub crop_x2: Option<u32>,
    pub crop_y1: Option<u32>,
    pub crop_y2: Option<u32>,
}

#[derive(Debug)]
pub struct AudioStreamProps {
    pub language: Option<String>,
    pub channels: Option<u32>,
    pub channel_layout: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct Subtitle {
    pub id: i64,
    pub video_id: i64,
    pub stream_index: Option<i64>,
    pub path: Option<Utf8PathBuf>,
    pub title: Option<String>,
    pub language: Option<String>,
    pub format: Option<String>,
    pub sdh: bool,
    pub forced: bool,
}

#[derive(Debug)]
pub struct VideoUserData {
    pub item_id: i64,
    pub position: f64,
    pub is_watched: bool,
    pub position_updated_at: Option<i64>,
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
                id: row.try_get("video_id")?,
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
            trailer: row.try_get("trailer")?,
            tmdb_id: row.try_get("tmdb_id")?,
            imdb_id: row.try_get("imdb_id")?,
            cast: vec![],
            director: row.try_get("director")?,
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
                    channels: row.try_get("a_channels")?,
                    channel_layout: row.try_get("a_channel_layout")?,
                }),
                StreamType::Video => StreamProps::Video(VideoStreamProps {
                    width: row.try_get("v_width")?,
                    height: row.try_get("v_height")?,
                    crop_x1: row.try_get("v_crop_x1")?,
                    crop_x2: row.try_get("v_crop_x2")?,
                    crop_y1: row.try_get("v_crop_y1")?,
                    crop_y2: row.try_get("v_crop_y2")?,
                }),
            },
        })
    }
}

impl<'r> FromRow<'r, SqliteRow> for VideoUserData {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(VideoUserData {
            item_id: row.try_get("item_id")?,
            position: row.try_get("position")?,
            is_watched: row.try_get("is_watched")?,
            position_updated_at: row.try_get("position_updated_at")?,
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
    "m.trailer",
    "m.tmdb_id",
    "m.imdb_id",
    "m.parent_id",
    "m.parent_index",
    "parent.name AS parent_name",
    "m.grandparent_id",
    "m.grandparent_index",
    "grandparent.name AS grandparent_name",
    "v.id AS video_id",
    "path",
    "duration",
    "format_name",
    "(SELECT name FROM crew JOIN people ON person_id = people.id WHERE item_id = m.id AND job = 'Director') AS director"
];

const STREAM_COLUMNS: &[&str] = &[
    "id",
    "video_id",
    "stream_index",
    "stream_type",
    "codec_name",
    "v_width",
    "v_height",
    "v_crop_x1",
    "v_crop_x2",
    "v_crop_y1",
    "v_crop_y2",
    "a_language",
    "a_channels",
    "a_channel_layout",
];

const SUBTITLE_COLUMNS: &[&str] = &[
    "id",
    "video_id",
    "stream_index",
    "path",
    "title",
    "language",
    "format",
    "sdh",
    "forced",
];

#[derive(Debug, Default)]
pub struct Query<'a> {
    pub ids: Option<&'a [i64]>,
    pub item_types: &'a [MediaItemType],
    pub parent_id: Option<i64>,
    pub grandparent_id: Option<i64>,
    pub collection_id: Option<i64>,
    pub name: Option<&'a str>,
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
    tracing::debug!("querying with params {query:?}");

    let mut args = QueryArguments::default();
    let mut conditions = vec![];
    let mut joins = vec![
        Join::left("video_files AS v ON v.item_id = m.id"),
        Join::left("media_items AS parent ON parent.id = m.parent_id"),
        Join::left("media_items AS grandparent ON grandparent.id = m.grandparent_id"),
    ];

    if let Some(ids) = query.ids {
        conditions.push(format!("m.id IN ({})", sql::Placeholders(ids.len())));
        for index in ids {
            args.add(index)?;
        }
    }

    if !query.item_types.is_empty() {
        let placeholders = sql::Placeholders(query.item_types.len());
        conditions.push(format!("m.item_type IN ({placeholders})"));
        for item_type in query.item_types {
            args.add(item_type)?;
        }
    }

    if let Some(id) = query.parent_id {
        conditions.push("m.parent_id = ?".to_owned());
        args.add(id)?;
    }

    if let Some(id) = query.grandparent_id {
        conditions.push("m.grandparent_id = ?".to_owned());
        args.add(id)?;
    }

    if let Some(id) = query.collection_id {
        joins.push(Join::left(
            "collections_media_items AS c ON c.item_id = m.id",
        ));
        conditions.push("c.collection_id = ?".to_owned());
        args.add(id)?;
    }

    if let Some(name) = query.name {
        conditions.push("m.name LIKE concat('%', ?, '%')".to_owned());
        args.add(name)?;
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

    let mut items: Vec<MediaItem> = sqlx::query_as_with(&sql, args.clone().into_inner())
        .fetch_all(&mut *conn)
        .await?;

    let condition = format!("mg.item_id IN ({})", sql::Placeholders(items.len()));
    let sql = sql::select("genres AS g")
        .columns(&["mg.item_id, g.name"])
        .joins(&[Join::inner(
            "media_items_genres AS mg ON mg.genre_id = g.id",
        )])
        .condition(&condition)
        .to_sql();

    let mut genres = sqlx::query_as_with::<_, (i64, String), _>(&sql, args.clone().into_inner())
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

    #[derive(FromRow)]
    struct CastMemberRow {
        item_id: i64,
        name: String,
        profile: Option<String>,
        character: Option<String>,
    }

    let condition = format!("c.item_id IN ({})", sql::Placeholders(items.len()));
    let sql = sql::select("cast AS c")
        .joins(&[Join::inner("people AS p ON c.person_id = p.id")])
        .columns(&["c.item_id", "p.name", "p.profile", "c.character"])
        .condition(&condition)
        .order_by(&["c.idx"])
        .to_sql();

    let mut cast = sqlx::query_as_with::<_, CastMemberRow, _>(&sql, args.into_inner())
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .into_grouping_map_by(|cast_member: &CastMemberRow| cast_member.item_id)
        .aggregate(|acc: Option<Vec<CastMember>>, _, cast_member| {
            let mut acc = acc.unwrap_or_default();
            acc.push(CastMember {
                name: cast_member.name,
                character: cast_member.character,
                profile: cast_member.profile,
            });
            Some(acc)
        });

    let video_ids = items
        .iter()
        .filter_map(|item| item.video_file.as_ref())
        .map(|item| item.id)
        .collect_vec();

    let mut args = QueryArguments::default();
    for id in &video_ids {
        args.add(*id)?;
    }

    let condition = format!("video_id IN ({})", sql::Placeholders(video_ids.len()));

    let sql = sql::select("video_file_streams")
        .columns(STREAM_COLUMNS)
        .condition(&condition)
        .to_sql();

    let mut streams = sqlx::query_as_with(&sql, args.clone().into_inner())
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .into_group_map_by(|stream: &Stream| stream.video_id);

    let sql = sql::select("subtitles")
        .columns(SUBTITLE_COLUMNS)
        .condition(&condition)
        .to_sql();

    let mut subtitles = sqlx::query_as_with(&sql, args.clone().into_inner())
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .into_group_map_by(|sub: &Subtitle| sub.video_id);

    for item in &mut items {
        item.genres = genres.remove(&item.id).unwrap_or_default();
        item.cast = cast.remove(&item.id).unwrap_or_default();

        if let Some(video) = &mut item.video_file {
            video.streams = streams.remove(&video.id).unwrap_or_default();
            video.subtitles = subtitles.remove(&video.id).unwrap_or_default();
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
    user_id: i64,
    limit: Option<u32>,
) -> eyre::Result<Vec<i64>> {
    // This beautiful query does two things:
    // - for movies, we grab ids of all movies where the user position is within the "currently watching" range
    // - for each show, we grab the last episode that was watched; if that episode was finished, then we instead
    //   get the next episode if it exists
    let mut sql = "
        SELECT id, position_updated_at FROM (
            SELECT m.id AS id, u.position_updated_at AS position_updated_at FROM movies AS m
            JOIN video_files AS v ON v.item_id = m.id
            LEFT JOIN media_item_user_data AS u ON m.id = u.item_id AND u.user_id = ?1
            WHERE u.position > (0.05 * v.duration) AND u.position < (0.9 * v.duration) AND u.position_updated_at IS NOT NULL
        )
        UNION
        SELECT id, position_updated_at FROM (
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
            ) AS id, MAX(position_updated_at) AS position_updated_at FROM episodes AS e
            JOIN seasons AS season ON season.id = e.season_id
            JOIN shows AS show ON show.id = season.show_id
            JOIN video_files AS v ON v.item_id = e.id
            LEFT JOIN media_item_user_data AS u ON e.id = u.item_id AND u.user_id = ?1
            WHERE u.position > (0.05 * v.duration) AND u.position_updated_at IS NOT NULL
            GROUP BY show.id
        )
        WHERE id IS NOT NULL
        ORDER BY position_updated_at DESC
    ".to_owned();

    if let Some(limit) = limit {
        write!(sql, "LIMIT {limit}").unwrap();
    }

    let media_ids = sqlx::query_scalar(&sql)
        .bind(user_id)
        .fetch_all(&mut *conn)
        .await?;

    Ok(media_ids)
}

pub async fn get_recently_added_movies(
    conn: &mut SqliteConnection,
    user_id: i64,
) -> eyre::Result<Vec<i64>> {
    let sql = sql::select("movies AS m")
        .columns(&["m.id"])
        .joins(&[Join::left(
            "media_item_user_data AS u ON u.item_id = m.id AND u.user_id = ?1",
        )])
        .condition("COALESCE(u.is_watched, 0) = 0")
        .order_by(&["added_at DESC", "name"])
        .limit(30)
        .to_sql();

    let movie_ids = sqlx::query_scalar(&sql)
        .bind(user_id)
        .fetch_all(&mut *conn)
        .await?;

    Ok(movie_ids)
}

pub async fn get_recently_updated_shows(
    conn: &mut SqliteConnection,
    user_id: i64,
) -> eyre::Result<Vec<i64>> {
    let condition = "
        (
            SELECT COUNT(*)
            FROM episodes AS episode
            LEFT JOIN media_item_user_data AS u ON u.item_id = episode.id AND u.user_id = ?1
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

    let show_ids = sqlx::query_scalar(&sql)
        .bind(user_id)
        .fetch_all(conn)
        .await?;

    Ok(show_ids)
}

pub async fn get_user_data_for_video(
    conn: &mut SqliteConnection,
    user_id: i64,
    id: i64,
) -> eyre::Result<Option<VideoUserData>> {
    Ok(get_user_data_for_videos(conn, user_id, &[id])
        .await?
        .into_iter()
        .next())
}

pub async fn get_user_data_for_videos(
    conn: &mut SqliteConnection,
    user_id: i64,
    ids: &[i64],
) -> eyre::Result<Vec<VideoUserData>> {
    let mut args = QueryArguments::default();
    args.add(user_id)?;
    for index in ids {
        args.add(index)?;
    }

    let sql = sql::select("video_files AS v")
        .joins(&[Join::left(
            "media_item_user_data AS u ON u.item_id = v.item_id AND u.user_id = ?",
        )])
        .columns(&[
            "v.item_id",
            "COALESCE(position, 0.0) AS position",
            "COALESCE(is_watched, 0) AS is_watched",
            "position_updated_at",
        ])
        .condition(&format!("v.item_id IN ({})", sql::Placeholders(ids.len())))
        .to_sql();

    Ok(sqlx::query_as_with(&sql, args.into_inner())
        .fetch_all(conn)
        .await?)
}

pub async fn get_user_data_for_collections(
    conn: &mut SqliteConnection,
    user_id: i64,
    ids: &[i64],
) -> eyre::Result<Vec<CollectionUserData>> {
    let mut args = QueryArguments::default();
    args.add(user_id)?;
    for index in ids {
        args.add(index)?;
    }

    let mut placeholders = String::new();
    for i in 2..=ids.len() + 1 {
        write!(placeholders, "?{i}")?;
        if i != ids.len() + 1 {
            write!(placeholders, ",")?;
        }
    }

    #[rustfmt::skip]
    let sql = format!("
        SELECT v.parent_id AS item_id, COUNT(*) AS unwatched
        FROM media_items AS v
        LEFT JOIN media_item_user_data AS u ON u.item_id = v.id AND u.user_id = ?1
        WHERE v.id IN (SELECT item_id FROM video_files)
            AND v.parent_id IN ({placeholders})
            AND COALESCE(u.is_watched, 0) = 0
        GROUP BY v.parent_id
        UNION
        SELECT v.grandparent_id AS item_id, COUNT(*) AS unwatched
        FROM media_items AS v
        LEFT JOIN media_item_user_data AS u ON u.item_id = v.id AND u.user_id = ?1
        WHERE v.id IN (SELECT item_id FROM video_files)
            AND v.grandparent_id IN ({placeholders})
            AND COALESCE(u.is_watched, 0) = 0
        GROUP BY v.grandparent_id
    ");

    Ok(sqlx::query_as_with(&sql, args.into_inner())
        .fetch_all(conn)
        .await?)
}

#[derive(Debug, Default)]
pub struct UpdateMetadata<'a> {
    pub name: Option<&'a str>,
    pub overview: Option<Option<&'a str>>,
    pub start_date: Option<Option<i64>>,
    pub end_date: Option<Option<i64>>,
    pub poster: Option<Option<String>>,
    pub backdrop: Option<Option<String>>,
    pub thumbnail: Option<Option<String>>,
    pub age_rating: Option<Option<&'a str>>,
    pub genres: Option<&'a [&'a str]>,
    pub cast: Option<&'a [UpdateCastMember<'a>]>,
    pub crew: Option<&'a [UpdateCrewMember<'a>]>,
    pub trailer: Option<Option<&'a str>>,
    pub tmdb_id: Option<Option<i32>>,
    pub imdb_id: Option<Option<&'a str>>,
    pub metadata_provider: Option<Option<MetadataProvider>>,
    pub metadata_provider_key: Option<Option<&'a str>>,
}

#[derive(Debug)]
pub struct UpdateCastMember<'a> {
    pub person_id: i64,
    pub idx: u32,
    pub character: Option<&'a str>,
}

#[derive(Debug)]
pub struct UpdateCrewMember<'a> {
    pub person_id: i64,
    pub department: &'a str,
    pub job: &'a str,
}

pub async fn update_metadata(
    conn: &mut SqliteConnection,
    id: i64,
    data: UpdateMetadata<'_>,
) -> eyre::Result<()> {
    let mut columns = vec![];
    let mut values = vec![];
    let mut args = QueryArguments::default();

    macro_rules! collect {
        ($field:ident) => {
            if let Some($field) = data.$field {
                columns.push(stringify!($field));
                values.push("?");
                args.add($field)?;
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
        trailer,
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
            args.add(img.map(|img| img.to_string()))?;
        }
    }

    columns.push("metadata_updated_at");
    values.push("(strftime('%s', 'now'))");

    args.add(id)?;

    let sql = sql::update("media_items")
        .columns(&columns)
        .values(&values)
        .condition("id = ?")
        .to_sql();

    let mut tx = conn.begin().await?;

    sqlx::query_with(&sql, args.into_inner())
        .execute(&mut *tx)
        .await?;

    if let Some(genres) = data.genres {
        let genre_ids = if !genres.is_empty() {
            let mut args = QueryArguments::default();
            let mut placeholders = String::new();

            for (i, &genre) in genres.iter().enumerate() {
                args.add(genre)?;
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

            let genre_ids: Vec<i64> = sqlx::query_scalar_with(&sql, args.into_inner())
                .fetch_all(&mut *tx)
                .await?;

            genre_ids
        } else {
            vec![]
        };

        let mut args = QueryArguments::default();
        let mut placeholders = String::new();

        for (i, genre_id) in genre_ids.iter().enumerate() {
            args.add(id)?;
            args.add(genre_id)?;
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

        sqlx::query_with(&sql, args.into_inner())
            .execute(&mut *tx)
            .await
            .wrap_err_with(|| eyre!("query failed: {sql}"))?;

        let placeholders = sql::Placeholders(genre_ids.len());
        let mut args = QueryArguments::default();
        for genre_id in &genre_ids {
            args.add(genre_id)?;
        }

        #[rustfmt::skip]
        let sql = format!("
            DELETE FROM media_items_genres
            WHERE item_id = ? AND genre_id NOT IN ({placeholders})
        ");

        sqlx::query_with(&sql, args.into_inner())
            .execute(&mut *tx)
            .await?;
    }

    if let Some(cast) = data.cast
        && !cast.is_empty()
    {
        let mut args = QueryArguments::default();
        let mut placeholders = String::new();

        for (i, cast_member) in cast.iter().enumerate() {
            args.add(id)?;
            args.add(cast_member.person_id)?;
            args.add(cast_member.idx)?;
            args.add(cast_member.character)?;
            placeholders += "(?, ?, ?, ?)";
            if i < cast.len() - 1 {
                placeholders += ",";
            }
        }

        #[rustfmt::skip]
        let sql = format!("
            INSERT INTO cast (item_id, person_id, idx, character) VALUES {placeholders}
            ON CONFLICT DO UPDATE SET
                idx = excluded.idx,
                character = excluded.character
        ");

        sqlx::query_with(&sql, args.into_inner())
            .execute(&mut *tx)
            .await
            .wrap_err_with(|| eyre!("query failed: {sql}"))?;

        let placeholders = sql::Placeholders(cast.len());
        let mut args = QueryArguments::default();
        for cast_member in cast {
            args.add(cast_member.person_id)?;
        }

        #[rustfmt::skip]
        let sql = format!("
            DELETE FROM cast
            WHERE item_id = ? AND person_id NOT IN ({placeholders})
        ");

        sqlx::query_with(&sql, args.into_inner())
            .execute(&mut *tx)
            .await?;
    }

    if let Some(crew) = data.crew
        && !crew.is_empty()
    {
        let mut args = QueryArguments::default();
        let mut placeholders = String::new();

        for (i, crew_member) in crew.iter().enumerate() {
            args.add(id)?;
            args.add(crew_member.person_id)?;
            args.add(crew_member.department)?;
            args.add(crew_member.job)?;
            placeholders += "(?, ?, ?, ?)";
            if i < crew.len() - 1 {
                placeholders += ",";
            }
        }

        #[rustfmt::skip]
        let sql = format!("
            INSERT INTO crew (item_id, person_id, department, job) VALUES {placeholders}
            ON CONFLICT DO UPDATE SET
                department = excluded.department,
                job = excluded.job
        ");

        sqlx::query_with(&sql, args.into_inner())
            .execute(&mut *tx)
            .await?;

        let mut args = QueryArguments::default();

        let mut sql = "
            DELETE FROM crew
            WHERE item_id = ? AND (person_id, department, job) NOT IN (
        "
        .to_string();

        for (i, crew_member) in crew.iter().enumerate() {
            args.add(crew_member.person_id)?;
            args.add(crew_member.department)?;
            args.add(crew_member.job)?;
            write!(sql, "({})", sql::Placeholders(3))?;
            if i < crew.len() - 1 {
                sql += ",";
            }
        }

        sql += ")";

        sqlx::query_with(&sql, args.into_inner())
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(())
}

/// Deletes a media item from the database, as well as any associated objects.
///
/// This should be called from within a transaction.
pub async fn remove(conn: &mut SqliteConnection, id: i64) -> eyre::Result<()> {
    sqlx::query("DELETE FROM media_item_user_data WHERE item_id = ?")
        .bind(id)
        .execute(&mut *conn)
        .await?;

    sqlx::query("DELETE FROM subtitles WHERE video_id = ?")
        .bind(id)
        .execute(&mut *conn)
        .await?;

    sqlx::query("DELETE FROM indexed_paths WHERE item_id = ?")
        .bind(id)
        .execute(&mut *conn)
        .await?;

    sqlx::query("DELETE FROM collections_media_items WHERE item_id = ?")
        .bind(id)
        .execute(&mut *conn)
        .await?;

    sqlx::query("DELETE FROM media_items_genres WHERE item_id = ?")
        .bind(id)
        .execute(&mut *conn)
        .await?;

    sqlx::query("DELETE FROM cast WHERE item_id = ?")
        .bind(id)
        .execute(&mut *conn)
        .await?;

    sqlx::query("DELETE FROM crew WHERE item_id = ?")
        .bind(id)
        .execute(&mut *conn)
        .await?;

    sqlx::query("DELETE FROM media_items WHERE id = ?")
        .bind(id)
        .execute(&mut *conn)
        .await?;

    Ok(())
}
