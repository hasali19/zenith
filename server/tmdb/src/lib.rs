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
pub struct MovieResponse {
    pub id: i32,
    pub title: String,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
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
}

#[derive(Debug, serde::Deserialize)]
pub struct TvSeasonResponse {
    pub id: i32,
    pub name: Option<String>,
    pub air_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvEpisodeResponse {
    pub id: i32,
    pub name: Option<String>,
    pub air_date: Option<String>,
    pub overview: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvEpisodeImagesResponse {
    pub id: i32,
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
        let url = self.url(&format!("movie/{id}"));
        self.get_json(url).await
    }

    pub async fn get_tv_show(&self, id: i32) -> eyre::Result<TvShowResponse> {
        let url = self.url(&format!("tv/{id}"));
        self.get_json(url).await
    }

    pub async fn get_tv_season(&self, tv_id: i32, season: i32) -> eyre::Result<TvSeasonResponse> {
        let url = self.url(&format!("tv/{tv_id}/season/{season}"));
        self.get_json(url).await
    }

    pub async fn get_tv_episode(
        &self,
        tv_id: i32,
        season: i32,
        episode: i32,
    ) -> eyre::Result<TvEpisodeResponse> {
        let path = format!("tv/{tv_id}/season/{season}/episode/{episode}");
        let url = self.url(&path);
        self.get_json(url).await
    }

    pub async fn get_tv_episode_images(
        &self,
        tv_id: i32,
        season: i32,
        episode: i32,
    ) -> eyre::Result<TvEpisodeImagesResponse> {
        let path = format!("tv/{tv_id}/season/{season}/episode/{episode}/images");
        let url = self.url(&path);
        self.get_json(url).await
    }

    fn url(&self, path: &str) -> Url {
        let mut url = Url::parse("https://api.themoviedb.org/3").unwrap();
        url.path_segments_mut().unwrap().extend(path.split('/'));
        url.query_pairs_mut().append_pair("api_key", &self.api_key);
        url
    }

    async fn get_json<T: DeserializeOwned>(&self, url: Url) -> eyre::Result<T> {
        Ok(self.client.get(url).send().await?.json().await?)
    }
}
