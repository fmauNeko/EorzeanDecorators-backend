mod root;

use std::time::Duration;

use root::*;

use axum::{routing::MethodRouter, Router};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::app::AppState;

const ROUTES: &[&dyn Route] = &[&Root {}];

pub fn create_router() -> Router<AppState> {
  let mut router = Router::new();

  for route in ROUTES {
    router = router.route(route.path(), route.method_router())
  }

  router = router
    .layer(TraceLayer::new_for_http())
    .layer(TimeoutLayer::new(Duration::from_secs(10)));

  return router;
}

pub trait Route {
  fn path(&self) -> &'static str;
  fn method_router(&self) -> MethodRouter<AppState>;
}
