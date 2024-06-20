use axum::routing::MethodRouter;

use super::Route;

pub struct Root {}

impl Route for Root {
  fn path(&self) -> &'static str {
    "/"
  }

  fn method_router(&self) -> MethodRouter {
    use axum::routing::get;
    get(|| async { "Hello, World!" })
  }
}
