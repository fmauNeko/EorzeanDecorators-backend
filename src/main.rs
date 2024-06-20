mod app;
mod routes;

#[tokio::main]
async fn main() {
  // build our application with a single route
  let router = app::create_router();

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("[::]:3000").await.unwrap();
  axum::serve(listener, router).await.unwrap();
}
