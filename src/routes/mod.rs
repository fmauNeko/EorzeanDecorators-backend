mod root;

use root::*;

use axum::{routing::MethodRouter, Router};

const ROUTES: &[&dyn Route] = &[&Root {}];

pub fn register_routes(mut router: Router) -> Router {
  for route in ROUTES {
    router = router.route(route.path(), route.method_router())
  }
  router
}

pub trait Route {
  fn path(&self) -> &'static str;
  fn method_router(&self) -> MethodRouter;
}
