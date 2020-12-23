pub fn get_image_url(poster: &str) -> String {
    let (img_type, value) = poster.split_once('|').unwrap();
    match img_type {
        // TODO: Don't use hard coded tmdb urls
        "tmdb.poster" => format!("https://image.tmdb.org/t/p/w342{}", value),
        "tmdb.backdrop" => format!("https://image.tmdb.org/t/p/original{}", value),
        "tmdb.still" => format!("https://image.tmdb.org/t/p/original{}", value),
        _ => unreachable!(),
    }
}
