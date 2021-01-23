use hyper::{Body, Response};

use crate::server::App;
use crate::AppState;

pub fn configure(app: &mut App<AppState>) {
    app.post("/api/library", |mut state: AppState, _| async move {
        state.sync_service.start_full_sync();
        Response::new(Body::empty())
    });
}
