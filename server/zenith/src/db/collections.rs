use axum_codegen::Reflect;
use serde::Serialize;

#[derive(Serialize, Reflect)]
pub struct CollectionUserData {
    pub unwatched: u32,
}
