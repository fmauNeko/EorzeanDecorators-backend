use app::AppState;
use tokio::{net::TcpListener, signal};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod routes;

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "eorzean_decorators_backend=debug,tower_http=debug,axum::rejection=trace".into()
      }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  let state = AppState::default();

  let router = routes::create_router().with_state(state);

  // run our app with hyper, listening globally on port 3000
  let listener = TcpListener::bind("[::]:3000").await.unwrap();
  tracing::info!("Listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, router)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("Failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("Failed to install signal handler")
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
