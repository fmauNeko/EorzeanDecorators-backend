use axum::Router;

use crate::routes;

pub fn create_router() -> Router {
  let router = Router::new();
  return routes::register_routes(router);
}
