use camino::Utf8PathBuf;
use serde::Serialize;
use speq::Reflect;

use crate::db::media::MediaItemType;
use crate::{db, utils};

#[derive(Serialize, Reflect)]
pub struct MediaItem {
    pub id: i64,
    #[serde(rename = "type")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grandparent: Option<Parent>,
    pub external_ids: ExternalIds,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_file: Option<VideoFile>,
    pub user_data: Option<UserData>,
}

#[derive(Serialize, Reflect)]
pub struct Parent {
    pub id: i64,
    pub index: u32,
    pub name: String,
}

#[derive(Serialize, Reflect)]
pub struct ExternalIds {
    pub tmdb: Option<i32>,
    pub imdb: Option<String>,
}

#[derive(Serialize, Reflect)]
pub struct VideoFile {
    pub path: Utf8PathBuf,
    pub duration: Option<f64>,
    pub format: Option<String>,
    pub streams: Vec<Stream>,
    pub subtitles: Vec<Subtitle>,
}

#[derive(Serialize, Reflect)]
pub struct Stream {
    pub id: i64,
    pub index: u32,
    pub codec: String,
    #[serde(flatten)]
    pub props: StreamProps,
}

#[derive(Serialize, Reflect)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum StreamProps {
    Video(VideoStreamProps),
    Audio(AudioStreamProps),
}

#[derive(Serialize, Reflect)]
pub struct VideoStreamProps {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Reflect)]
pub struct AudioStreamProps {
    pub language: Option<String>,
    pub channels: Option<u32>,
    pub channel_layout: Option<String>,
}

#[derive(Serialize, Reflect)]
pub struct Subtitle {
    pub id: i64,
    pub stream_index: Option<i64>,
    pub path: Option<Utf8PathBuf>,
    pub title: Option<String>,
    pub language: Option<String>,
    pub format: Option<String>,
    pub sdh: bool,
    pub forced: bool,
}

#[derive(Serialize, Reflect)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum UserData {
    Collection {
        unwatched: u32,
    },
    Video {
        position: f64,
        is_watched: bool,
        last_watched_at: Option<i64>,
    },
}

impl From<db::items::MediaItem> for MediaItem {
    fn from(item: db::items::MediaItem) -> Self {
        MediaItem {
            id: item.id,
            kind: item.kind,
            name: item.name,
            overview: item.overview,
            start_date: item.start_date,
            end_date: item.end_date,
            poster: item.poster.map(utils::get_image_url),
            backdrop: item.backdrop.map(utils::get_image_url),
            thumbnail: item.thumbnail.map(utils::get_image_url),
            age_rating: item.age_rating,
            genres: item.genres,
            trailer: item.trailer,
            parent: item.parent.map(|parent| Parent {
                id: parent.id,
                index: parent.index,
                name: parent.name,
            }),
            grandparent: item.grandparent.map(|grandparent| Parent {
                id: grandparent.id,
                index: grandparent.index,
                name: grandparent.name,
            }),
            external_ids: ExternalIds {
                tmdb: item.tmdb_id,
                imdb: item.imdb_id,
            },
            video_file: item.video_file.map(|v| VideoFile {
                path: v.path,
                duration: v.duration,
                format: v.format_name,
                streams: v
                    .streams
                    .into_iter()
                    .map(|stream| Stream {
                        id: stream.id,
                        index: stream.index,
                        codec: stream.codec_name,
                        props: match stream.props {
                            db::items::StreamProps::Video(props) => {
                                StreamProps::Video(VideoStreamProps {
                                    width: props.width,
                                    height: props.height,
                                })
                            }
                            db::items::StreamProps::Audio(props) => {
                                StreamProps::Audio(AudioStreamProps {
                                    language: props.language,
                                    channels: props.channels,
                                    channel_layout: props.channel_layout,
                                })
                            }
                        },
                    })
                    .collect(),
                subtitles: v
                    .subtitles
                    .into_iter()
                    .map(|subtitle| Subtitle {
                        id: subtitle.id,
                        path: subtitle.path,
                        stream_index: subtitle.stream_index,
                        title: subtitle.title,
                        language: subtitle.language,
                        format: subtitle.format,
                        sdh: subtitle.sdh,
                        forced: subtitle.forced,
                    })
                    .collect(),
            }),
            user_data: None,
        }
    }
}

impl From<db::items::VideoUserData> for UserData {
    fn from(user_data: db::items::VideoUserData) -> Self {
        UserData::Video {
            position: user_data.position,
            is_watched: user_data.is_watched,
            last_watched_at: user_data.last_watched_at,
        }
    }
}

impl From<db::items::CollectionUserData> for UserData {
    fn from(user_data: db::items::CollectionUserData) -> Self {
        UserData::Collection {
            unwatched: user_data.unwatched,
        }
    }
}
