use serde::Serialize;
use speq::Reflect;

#[derive(Serialize, Reflect)]
pub struct CollectionUserData {
    pub unwatched: u32,
}
