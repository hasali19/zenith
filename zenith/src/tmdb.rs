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
}
