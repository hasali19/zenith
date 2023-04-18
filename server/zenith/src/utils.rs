use std::convert::TryFrom;

use db::media::{MediaImage, MediaImageSrcType, MediaImageType};

pub fn get_image_url(value: impl AsRef<str>) -> String {
    let image = MediaImage::try_from(value.as_ref()).unwrap();
    let src = image.src;
    match image.src_type {
        MediaImageSrcType::Local => todo!(),
        MediaImageSrcType::Tmdb => match image.img_type {
            // TODO: Don't use hard coded tmdb urls
            MediaImageType::Poster => format!("https://image.tmdb.org/t/p/w342{src}"),
            MediaImageType::Backdrop => format!("https://image.tmdb.org/t/p/original{src}"),
            MediaImageType::Thumbnail => format!("https://image.tmdb.org/t/p/original{src}"),
        },
    }
}
