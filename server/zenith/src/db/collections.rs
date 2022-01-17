use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct CollectionUserData {
    pub unwatched: u32,
}
