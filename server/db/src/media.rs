use std::convert::TryFrom;
use std::fmt::Display;

use eyre::eyre;
use sqlx::{SqliteConnection, Type};
use strum::FromRepr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Type)]
#[repr(i32)]
pub enum MediaItemType {
    Movie = 1,
    Show = 2,
    Season = 3,
    Episode = 4,
}

impl MediaItemType {
    pub fn is_video(&self) -> bool {
        matches!(self, MediaItemType::Movie | MediaItemType::Episode)
    }
}

#[derive(Clone, Copy, FromRepr)]
#[repr(i32)]
pub enum MediaImageType {
    Poster = 1,
    Backdrop = 2,
    Thumbnail = 3,
    Profile = 4,
}

impl TryFrom<i32> for MediaImageType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        MediaImageType::from_repr(value).ok_or(())
    }
}

#[derive(Clone, Copy, FromRepr)]
#[repr(i32)]
pub enum MediaImageSrcType {
    Local = 1,
    Tmdb = 2,
}

impl TryFrom<i32> for MediaImageSrcType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        MediaImageSrcType::from_repr(value).ok_or(())
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

impl Display for MediaImage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
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

#[derive(Type)]
#[sqlx(rename_all = "snake_case")]
pub enum MetadataProvider {
    Tmdb,
}
