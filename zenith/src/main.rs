use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use env_logger::Env;

use zenith::config::Config;
use zenith::db::Db;
use zenith::sync::SyncService;
use zenith::tmdb::TmdbClient;
use zenith::watcher::FileWatcher;
use zenith::{middleware, AppState};
use zenith_server::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::new().default_filter_or("info,sqlx::query=warn"));

    let config = Arc::new(Config::load("config.yml")?);
    let tmdb = TmdbClient::new(&config.tmdb.access_token);
    let db = Db::init(&config.database.path).await?;
    let sync_service = SyncService::new(db.clone(), tmdb, config.clone());

    let mut watcher = FileWatcher::spawn({
        let mut sync_service = sync_service.clone();
        move |_| {
            // Run sync anytime anything changes
            // TODO: Make this more clever
            sync_service.start_full_sync();
        }
    });

    watcher.watch(&config.libraries.movies);
    watcher.watch(&config.libraries.tv_shows);

    let mut app = App::new(AppState {
        config: config.clone(),
        db: db.clone(),
        sync_service,
    });

    let addr = SocketAddr::from_str(&format!("{}:{}", config.http.host, config.http.port))?;

    app.wrap(middleware::Logger);
    app.configure(zenith::api::configure);
    app.run(addr).await?;

    db.close().await;

    Ok(())
}
