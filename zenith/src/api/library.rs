use zenith_http::App;

use crate::AppState;

pub fn configure(app: &mut App<AppState>) {
    app.post("/api/library/sync", |mut state: AppState, _| async move {
        state.sync.start_full_sync();
    });
}
