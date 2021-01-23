use std::net::SocketAddr;
use std::sync::Arc;

use env_logger::Env;

use zenith::config::Config;
use zenith::db::Db;
use zenith::server::App;
use zenith::sync::SyncService;
use zenith::tmdb::TmdbClient;
use zenith::watcher::FileWatcher;
use zenith::AppState;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::new().default_filter_or("info,sqlx::query=warn"));

    let config = Arc::new(Config::load("config.yml")?);
    let tmdb = TmdbClient::new(&config.tmdb_access_token);
    let db = Db::init(config.db_path()).await?;
    let sync_service = SyncService::new(db.clone(), tmdb, config.clone());

    let mut watcher = FileWatcher::spawn({
        let mut sync_service = sync_service.clone();
        move |_| {
            // Run sync anytime anything changes
            // TODO: Make this more clever
            sync_service.start_full_sync();
        }
    });

    watcher.watch(&config.movie_path);
    watcher.watch(&config.tv_show_path);

    let mut app = App::new(AppState {
        config,
        db: db.clone(),
        sync_service,
    });

    app.configure(zenith::api::configure);
    app.run(SocketAddr::from(([0, 0, 0, 0], 8000))).await?;

    db.close().await;

    Ok(())
}
