use std::convert::TryFrom;

use eyre::eyre;
use serde::{Deserialize, Serialize};
use speq::Reflect;
use sqlx::{SqliteConnection, Type};

#[derive(Clone, Copy, Debug, Type, Serialize, Deserialize, Reflect)]
#[repr(i32)]
#[serde(rename_all = "snake_case")]
pub enum MediaItemType {
    Movie = 1,
    Show = 2,
    Season = 3,
    Episode = 4,
}

#[derive(Clone, Copy, Deserialize, Reflect)]
#[serde(rename_all = "snake_case")]
pub enum MediaImageType {
    Poster = 1,
    Backdrop = 2,
    Thumbnail = 3,
}

impl TryFrom<i32> for MediaImageType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MediaImageType::Poster),
            2 => Ok(MediaImageType::Backdrop),
            3 => Ok(MediaImageType::Thumbnail),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum MediaImageSrcType {
    Local = 1,
    Tmdb = 2,
}

impl TryFrom<i32> for MediaImageSrcType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MediaImageSrcType::Local),
            2 => Ok(MediaImageSrcType::Tmdb),
            _ => Err(()),
        }
    }
}

pub struct MediaImage<'a> {
    pub img_type: MediaImageType,
    pub src_type: MediaImageSrcType,
    pub src: &'a str,
}

impl<'a> TryFrom<&'a str> for MediaImage<'a> {
    type Error = eyre::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut iter = value.splitn(3, '|');

        let (img_type, src_type, src) = iter
            // Unpack iterator into an (&str, &str, &str)
            .next()
            .and_then(|v| Some((v, iter.next()?)))
            .and_then(|(v, w)| Some((v, w, iter.next()?)))
            .ok_or_else(|| eyre!("invalid image string"))?;

        let img_type = MediaImageType::try_from(img_type.parse::<i32>()?)
            .map_err(|_| eyre!("invalid image type"))?;

        let src_type = MediaImageSrcType::try_from(src_type.parse::<i32>()?)
            .map_err(|_| eyre!("invalid image src type"))?;

        Ok(MediaImage {
            img_type,
            src_type,
            src,
        })
    }
}

impl<'a> TryFrom<&'a String> for MediaImage<'a> {
    type Error = <MediaImage<'a> as TryFrom<&'a str>>::Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        MediaImage::try_from(value.as_str())
    }
}

impl<'a> ToString for MediaImage<'a> {
    fn to_string(&self) -> String {
        format!(
            "{}|{}|{}",
            self.img_type as i32, self.src_type as i32, self.src
        )
    }
}

pub async fn get_item_type(
    conn: &mut SqliteConnection,
    id: i64,
) -> eyre::Result<Option<MediaItemType>> {
    sqlx::query_scalar("SELECT item_type FROM media_items WHERE id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await
        .map_err(|e| e.into())
}
