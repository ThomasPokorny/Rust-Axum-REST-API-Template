use crate::domain::tweet::tweet_controller;
use crate::platform::service_controller;
use axum::Router;

mod domain;
mod platform;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(service_controller::router())
        .merge(tweet_controller::router());

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
