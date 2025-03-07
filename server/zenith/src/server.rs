use std::net::SocketAddr;
use std::time::Duration;

use axum::Router;
use hyper::Request;
use hyper::body::Incoming;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::sync::watch;
use tower::Service;
use tracing::debug;

pub async fn serve(addr: &SocketAddr, app: Router) -> eyre::Result<()> {
    tracing::info!("starting server at http://{addr}");

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind(addr).await.unwrap();

    // Create a watch channel to track tasks that are handling connections and wait
    // for them to complete.
    let (close_tx, close_rx) = watch::channel(());

    // Continuously accept new connections.
    loop {
        let (socket, remote_addr) = tokio::select! {
            // Either accept a new connection...
            result = listener.accept() => {
                result.unwrap()
            }
            // ...or wait to receive a shutdown signal and stop the accept loop.
            _ = shutdown_signal() => {
                debug!("signal received, not accepting new connections");
                break;
            }
        };

        debug!("connection {remote_addr} accepted");

        // We don't need to call `poll_ready` because `Router` is always ready.
        let tower_service = app.clone();

        // Clone the watch receiver and move it into the task.
        let close_rx = close_rx.clone();

        // Spawn a task to handle the connection. That way we can serve multiple
        // connections concurrently.
        tokio::spawn(async move {
            // Hyper has its own `AsyncRead` and `AsyncWrite` traits and doesn't use tokio.
            // `TokioIo` converts between them.
            let socket = TokioIo::new(socket);

            // Hyper also has its own `Service` trait and doesn't use tower. We can use
            // `hyper::service::service_fn` to create a hyper `Service` that calls our app
            // through `tower::Service::call`.
            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                // We have to clone `tower_service` because hyper's `Service` uses `&self`
                // whereas tower's `Service` requires `&mut self`.
                //
                // We don't need to call `poll_ready` since `Router` is always ready.
                tower_service.clone().call(request)
            });

            // `hyper_util::server::conn::auto::Builder` supports both http1 and http2 but
            // doesn't support graceful so we have to use hyper directly and
            // unfortunately pick between http1 and http2.
            let conn = hyper::server::conn::http1::Builder::new()
                .serve_connection(socket, hyper_service)
                // `with_upgrades` is required for websockets.
                .with_upgrades();

            // `graceful_shutdown` requires a pinned connection.
            let mut conn = std::pin::pin!(conn);

            loop {
                tokio::select! {
                    // Poll the connection. This completes when the client has closed the
                    // connection, graceful shutdown has completed, or we encounter a TCP error.
                    result = conn.as_mut() => {
                        if let Err(err) = result {
                            debug!("failed to serve connection: {err:#}");
                        }
                        break;
                    }
                    // Start graceful shutdown when we receive a shutdown signal.
                    //
                    // We use a loop to continue polling the connection to allow requests to finish
                    // after starting graceful shutdown. Our `Router` has `TimeoutLayer` so
                    // requests will finish after at most 10 seconds.
                    _ = shutdown_signal() => {
                        debug!("signal received, starting graceful shutdown");
                        conn.as_mut().graceful_shutdown();
                    }
                }
            }

            debug!("connection {remote_addr} closed");

            // Drop the watch receiver to signal to `main` that this task is done.
            drop(close_rx);
        });
    }

    // We only care about the watch receivers that were moved into the tasks so
    // close the residual receiver.
    drop(close_rx);

    // Close the listener to stop accepting new connections.
    drop(listener);

    // Wait for all tasks to complete.
    debug!("waiting for {} tasks to finish", close_tx.receiver_count());
    if (tokio::time::timeout(Duration::from_secs(5), close_tx.closed()).await).is_err() {
        tracing::warn!("tasks took too long to respond, forcing shutdown");
    }

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
