use std::convert::TryFrom;

use crate::db::media::{MediaImage, MediaImageSrcType, MediaImageType};

pub fn get_image_url(value: &str) -> String {
    let image = MediaImage::try_from(value).unwrap();
    match image.src_type {
        MediaImageSrcType::Local => todo!(),
        MediaImageSrcType::Tmdb => match image.img_type {
            // TODO: Don't use hard coded tmdb urls
            MediaImageType::Poster => format!("https://image.tmdb.org/t/p/w342{}", image.src),
            MediaImageType::Backdrop => format!("https://image.tmdb.org/t/p/original{}", image.src),
            MediaImageType::Thumbnail => {
                format!("https://image.tmdb.org/t/p/original{}", image.src)
            }
        },
    }
}
