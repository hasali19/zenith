use reqwest::header::{self, HeaderMap};
use reqwest::Client as HttpClient;
use url::Url;

pub struct TmdbClient {
    client: HttpClient,
}

pub struct MovieSearchQuery<'a> {
    pub title: &'a str,
    pub page: Option<i32>,
    pub primary_release_year: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct MovieSearchResponse {
    pub page: i32,
    pub results: Vec<MovieSearchResult>,
    pub total_results: i32,
    pub total_pages: i32,
}

#[derive(serde::Deserialize)]
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

#[derive(serde::Deserialize)]
pub struct TvShowSearchResponse {
    pub page: i32,
    pub results: Vec<TvShowSearchResult>,
    pub total_results: i32,
    pub total_pages: i32,
}

#[derive(serde::Deserialize)]
pub struct TvShowSearchResult {
    pub id: i32,
    pub name: String,
    pub first_air_date_year: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct TvSeasonResponse {
    pub id: i32,
    pub name: Option<String>,
    pub air_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct TvEpisodeResponse {
    pub id: i32,
    pub name: Option<String>,
    pub air_date: Option<String>,
    pub overview: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct TvEpisodeImagesResponse {
    pub id: i32,
    pub stills: Vec<Image>,
}

#[derive(serde::Deserialize)]
pub struct Image {
    pub aspect_ratio: f64,
    pub file_path: String,
    pub width: i32,
    pub height: i32,
}

impl TmdbClient {
    pub fn new(access_token: &str) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", access_token).parse().unwrap(),
        );

        TmdbClient {
            client: HttpClient::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn search_movies(
        &self,
        query: &MovieSearchQuery<'_>,
    ) -> eyre::Result<MovieSearchResponse> {
        let url = "https://api.themoviedb.org/3/search/movie";

        let mut url = Url::parse(url).unwrap();
        {
            let mut params = url.query_pairs_mut();

            params.append_pair("query", query.title);

            if let Some(page) = query.page {
                params.append_pair("page", &page.to_string());
            }

            if let Some(year) = query.primary_release_year {
                params.append_pair("primary_release_year", &year.to_string());
            }
        }

        Ok(self.client.get(url.as_str()).send().await?.json().await?)
    }

    pub async fn search_tv_shows(
        &self,
        query: &TvShowSearchQuery<'_>,
    ) -> eyre::Result<TvShowSearchResponse> {
        let url = "https://api.themoviedb.org/3/search/tv";

        let mut url = Url::parse(url).unwrap();
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

        Ok(self.client.get(url.as_str()).send().await?.json().await?)
    }

    pub async fn get_tv_season(&self, tv_id: i32, season: i32) -> eyre::Result<TvSeasonResponse> {
        let url = format!(
            "https://api.themoviedb.org/3/tv/{}/season/{}",
            tv_id, season
        );

        Ok(self.client.get(&url).send().await?.json().await?)
    }

    pub async fn get_tv_episode(
        &self,
        tv_id: i32,
        season: i32,
        episode: i32,
    ) -> eyre::Result<TvEpisodeResponse> {
        let url = format!(
            "https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}",
            tv_id, season, episode
        );

        Ok(self.client.get(&url).send().await?.json().await?)
    }

    pub async fn get_tv_episode_images(
        &self,
        tv_id: i32,
        season: i32,
        episode: i32,
    ) -> eyre::Result<TvEpisodeImagesResponse> {
        let url = format!(
            "https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}/images",
            tv_id, season, episode
        );

        Ok(self.client.get(&url).send().await?.json().await?)
    }
}
