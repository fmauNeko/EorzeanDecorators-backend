use axum::{extract::State, routing::MethodRouter};

use crate::app::AppState;

use super::Route;

pub struct Root {}

impl Route for Root {
  fn path(&self) -> &'static str {
    "/"
  }

  fn method_router(&self) -> MethodRouter<AppState> {
    use axum::routing::get;
    get(|State(state): State<AppState>| async move { format!("Hello, {}!", state.name) })
  }
}
