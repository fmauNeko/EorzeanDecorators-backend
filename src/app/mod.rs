#[derive(Clone)]
pub struct AppState {
  pub name: String,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      name: "Eorzean Decorators".to_string(),
    }
  }
}
