use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use env_logger::Env;

use zenith::config::Config;
use zenith::db::Db;
use zenith::library::MediaLibraryImpl;
use zenith::lifecycle::AppLifecycle;
use zenith::metadata::MetadataManager;
use zenith::sync::LibrarySync;
use zenith::tmdb::TmdbClient;
use zenith::watcher::FileWatcher;
use zenith::{middleware, AppState};
use zenith_http::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::new().default_filter_or("info,sqlx::query=warn"));

    let lifecycle = AppLifecycle::new();
    let config = Arc::new(Config::load("config.yml")?);
    let db = Db::init(&config.database.path).await?;
    let tmdb = TmdbClient::new(&config.tmdb.access_token);
    let metadata = MetadataManager::new(db.clone(), tmdb);
    let library = MediaLibraryImpl::new(db.clone(), metadata.clone());
    let mut sync = LibrarySync::new(library, config.clone(), &lifecycle);

    sync.start_full_sync();

    let mut watcher = FileWatcher::spawn({
        let mut sync = sync.clone();
        move |_| {
            // Run full sync anytime anything changes
            // TODO: Make this more clever
            sync.start_full_sync();
        }
    });

    watcher.watch(&config.libraries.movies);
    watcher.watch(&config.libraries.tv_shows);

    let mut app = App::new(AppState {
        config: config.clone(),
        db: db.clone(),
        sync,
        metadata,
    });

    let addr = SocketAddr::from_str(&format!("{}:{}", config.http.host, config.http.port))?;

    app.wrap(middleware::Logger);
    app.configure(zenith::api::configure);
    app.run(addr).await?;

    lifecycle.signal_stopped()?;

    db.close().await;

    Ok(())
}
