use std::convert::TryFrom;

use eyre::eyre;
use itertools::Itertools;
use serde::Serialize;
use sqlx::Type;

#[derive(Clone, Copy, Debug, Type, Serialize)]
#[repr(i32)]
pub enum MediaItemType {
    Movie = 1,
    TvShow = 2,
    TvSeason = 3,
    TvEpisode = 4,
}

#[derive(Clone, Copy)]
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
        let (img_type, src_type, src): (&str, &str, &str) = value
            .splitn(3, '|')
            .next_tuple()
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

impl<'a> ToString for MediaImage<'a> {
    fn to_string(&self) -> String {
        format!(
            "{}|{}|{}",
            self.img_type as i32, self.src_type as i32, self.src
        )
    }
}
