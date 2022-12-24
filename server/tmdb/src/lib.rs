use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use url::Url;

#[derive(Clone)]
pub struct TmdbClient {
    api_key: String,
    client: HttpClient,
}

pub struct MovieSearchQuery<'a> {
    pub title: &'a str,
    pub page: Option<i32>,
    pub year: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MovieSearchResponse {
    pub page: i32,
    pub results: Vec<MovieSearchResult>,
    pub total_results: i32,
    pub total_pages: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct MovieSearchResult {
    pub id: i32,
    pub title: String,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

pub struct TvShowSearchQuery<'a> {
    pub name: &'a str,
    pub page: Option<i32>,
    pub first_air_date_year: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvShowSearchResponse {
    pub page: i32,
    pub results: Vec<TvShowSearchResult>,
    pub total_results: i32,
    pub total_pages: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvShowSearchResult {
    pub id: i32,
    pub name: String,
    pub first_air_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ExternalIds {
    pub imdb_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MovieResponse {
    pub id: i32,
    pub title: String,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genres: Vec<Genre>,
    pub external_ids: ExternalIds,
    pub release_dates: MovieReleaseDatesResults,
}

#[derive(Debug, serde::Deserialize)]
pub struct MovieReleaseDatesResults {
    pub results: Vec<MovieReleaseDatesResult>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MovieReleaseDatesResult {
    pub iso_3166_1: String,
    pub release_dates: Vec<MovieReleaseDate>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MovieReleaseDate {
    pub certification: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvShowResponse {
    pub id: i32,
    pub name: String,
    pub first_air_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub last_air_date: Option<String>,
    pub imdb_id: Option<String>,
    pub genres: Vec<Genre>,
    pub external_ids: ExternalIds,
    pub content_ratings: TvShowContentRatings,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvShowContentRatings {
    pub results: Vec<TvShowContentRating>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvShowContentRating {
    pub iso_3166_1: String,
    pub rating: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvSeasonResponse {
    pub id: i32,
    pub name: Option<String>,
    pub air_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub external_ids: ExternalIds,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvEpisodeResponse {
    pub id: i32,
    pub name: Option<String>,
    pub air_date: Option<String>,
    pub overview: Option<String>,
    pub external_ids: ExternalIds,
    pub images: TvEpisodeImages,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvEpisodeImages {
    pub stills: Vec<Image>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Image {
    pub aspect_ratio: f64,
    pub file_path: String,
    pub width: i32,
    pub height: i32,
}

impl TmdbClient {
    pub fn new(api_key: &str) -> Self {
        TmdbClient {
            api_key: api_key.to_owned(),
            client: HttpClient::new(),
        }
    }

    pub async fn search_movies(
        &self,
        query: &MovieSearchQuery<'_>,
    ) -> eyre::Result<MovieSearchResponse> {
        let mut url = self.url("search/movie");
        {
            let mut params = url.query_pairs_mut();

            params.append_pair("query", query.title);

            if let Some(page) = query.page {
                params.append_pair("page", &page.to_string());
            }

            if let Some(year) = query.year {
                params.append_pair("year", &year.to_string());
            }
        }

        self.get_json(url).await
    }

    pub async fn search_tv_shows(
        &self,
        query: &TvShowSearchQuery<'_>,
    ) -> eyre::Result<TvShowSearchResponse> {
        let mut url = self.url("search/tv");
        {
            let mut params = url.query_pairs_mut();

            params.append_pair("query", query.name);

            if let Some(page) = query.page {
                params.append_pair("page", &page.to_string());
            }

            if let Some(year) = query.first_air_date_year {
                params.append_pair("first_air_date_year", &year.to_string());
            }
        }

        self.get_json(url).await
    }

    pub async fn get_movie(&self, id: i32) -> eyre::Result<MovieResponse> {
        let mut url = self.url(&format!("movie/{id}"));
        url.query_pairs_mut()
            .append_pair("append_to_response", "external_ids,release_dates");
        self.get_json(url).await
    }

    pub async fn get_tv_show(&self, id: i32) -> eyre::Result<TvShowResponse> {
        let mut url = self.url(&format!("tv/{id}"));
        url.query_pairs_mut()
            .append_pair("append_to_response", "external_ids,content_ratings");
        self.get_json(url).await
    }

    pub async fn get_tv_season(&self, tv_id: i32, season: i32) -> eyre::Result<TvSeasonResponse> {
        let mut url = self.url(&format!("tv/{tv_id}/season/{season}"));
        url.query_pairs_mut()
            .append_pair("append_to_response", "external_ids");
        self.get_json(url).await
    }

    pub async fn get_tv_episode(
        &self,
        tv_id: i32,
        season: i32,
        episode: i32,
    ) -> eyre::Result<TvEpisodeResponse> {
        let path = format!("tv/{tv_id}/season/{season}/episode/{episode}");
        let mut url = self.url(&path);
        url.query_pairs_mut()
            .append_pair("append_to_response", "external_ids,images");
        self.get_json(url).await
    }

    fn url(&self, path: &str) -> Url {
        let mut url = Url::parse("https://api.themoviedb.org/3").unwrap();
        url.path_segments_mut().unwrap().extend(path.split('/'));
        url.query_pairs_mut().append_pair("api_key", &self.api_key);
        url
    }

    async fn get_json<T: DeserializeOwned>(&self, url: Url) -> eyre::Result<T> {
        let res = self.client.get(url).send().await?;
        Ok(res.error_for_status()?.json().await?)
    }
}
