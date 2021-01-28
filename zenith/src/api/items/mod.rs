use zenith_server::App;

use crate::AppState;

mod list;
mod single;

pub fn configure(app: &mut App<AppState>) {
    app.get("/api/items", list::get);
    app.get("/api/items/:id", single::get);
    app.get("/api/items/:id/children", list::get_children);
    app.post("/api/items/:id/metadata/refresh", single::refresh_metadata);
}
