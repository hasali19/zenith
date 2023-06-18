use eyre::Context;
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
    pub videos: VideoResults,
    pub credits: Credits,
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
    pub videos: VideoResults,
    pub aggregate_credits: Credits,
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
    pub videos: VideoResults,
    pub aggregate_credits: Credits,
}

#[derive(Debug, serde::Deserialize)]
pub struct TvEpisodeResponse {
    pub id: i32,
    pub name: Option<String>,
    pub air_date: Option<String>,
    pub overview: Option<String>,
    pub external_ids: ExternalIds,
    pub images: TvEpisodeImages,
    pub videos: VideoResults,
    pub credits: Credits,
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

#[derive(Debug, serde::Deserialize)]
pub struct VideoResults {
    pub results: Vec<Video>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Video {
    pub id: String,
    #[serde(rename = "type")]
    pub video_type: VideoType,
    pub site: VideoSite,
    pub key: String,
    pub name: Option<String>,
    pub size: Option<i32>,
    #[serde(default)]
    pub official: bool,
    pub published_at: Option<String>,
    pub iso_639_1: Option<String>,
    pub iso_3166_1: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
pub enum VideoType {
    #[serde(rename = "Behind the Scenes")]
    BehindTheScenes,
    Bloopers,
    Clip,
    Featurette,
    Recap,
    Teaser,
    Trailer,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
pub enum VideoSite {
    #[serde(rename = "YouTube")]
    YouTube,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, serde::Deserialize)]
pub struct Credits {
    pub cast: Vec<CastMember>,
    pub crew: Vec<CrewMember>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CastMember {
    pub id: i32,
    pub name: String,
    pub profile_path: Option<String>,
    pub character: Option<String>,
    pub roles: Option<Vec<CastMemberRole>>,
    pub order: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct CastMemberRole {
    pub character: Option<String>,
    pub episode_count: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct CrewMember {
    pub id: i32,
    pub name: String,
    pub profile_path: Option<String>,
    pub department: Option<String>,
    pub job: Option<String>,
    pub jobs: Option<Vec<CrewMemberJob>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CrewMemberJob {
    pub job: String,
    pub episode_count: i32,
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
        url.query_pairs_mut().append_pair(
            "append_to_response",
            "external_ids,release_dates,videos,credits",
        );
        self.get_json(url).await
    }

    pub async fn get_tv_show(&self, id: i32) -> eyre::Result<TvShowResponse> {
        let mut url = self.url(&format!("tv/{id}"));
        url.query_pairs_mut().append_pair(
            "append_to_response",
            "external_ids,content_ratings,videos,aggregate_credits",
        );
        self.get_json(url).await
    }

    pub async fn get_tv_season(&self, tv_id: i32, season: i32) -> eyre::Result<TvSeasonResponse> {
        let mut url = self.url(&format!("tv/{tv_id}/season/{season}"));
        url.query_pairs_mut().append_pair(
            "append_to_response",
            "external_ids,videos,aggregate_credits",
        );
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
            .append_pair("append_to_response", "external_ids,images,videos,credits");
        self.get_json(url).await
    }

    fn url(&self, path: &str) -> Url {
        let mut url = Url::parse("https://api.themoviedb.org/3").unwrap();
        url.path_segments_mut().unwrap().extend(path.split('/'));
        url.query_pairs_mut().append_pair("api_key", &self.api_key);
        url
    }

    async fn get_json<T: DeserializeOwned>(&self, url: Url) -> eyre::Result<T> {
        tracing::debug!("requesting {url}");
        let res = self.client.get(url).send().await?;
        let bytes = res.error_for_status()?.bytes().await?;
        serde_json::from_slice(&bytes).wrap_err_with(|| {
            let value = String::from_utf8_lossy(&bytes);
            format!("failed to deserialize json: {value}")
        })
    }
}
