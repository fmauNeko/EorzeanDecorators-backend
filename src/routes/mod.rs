mod root;

use std::sync::Arc;

use root::*;

use axum::{routing::MethodRouter, Router};

use crate::app::AppState;

const ROUTES: &[&dyn Route] = &[&Root {}];

pub fn create_router() -> Router {
  let mut router = Router::new().with_state(Arc::new(AppState {}));

  for route in ROUTES {
    router = router.route(route.path(), route.method_router())
  }
  return router;
}

pub trait Route {
  fn path(&self) -> &'static str;
  fn method_router(&self) -> MethodRouter;
}
