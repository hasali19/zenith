use serde::Serialize;

#[derive(Serialize)]
pub struct CollectionUserData {
    pub unwatched: u32,
}
