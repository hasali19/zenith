use serde::Serialize;

#[derive(Serialize)]
pub struct ExternalIds {
    pub tmdb: Option<i32>,
}
