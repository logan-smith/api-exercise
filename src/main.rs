use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use crate::config::CONFIG;
use crate::handlers::health::get_health_endpoint;
use crate::handlers::post::{create_post_endpoint, get_post_endpoint};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

pub mod config;
pub mod errors;
pub mod handlers;
pub mod tests;
pub mod validate;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let config = CONFIG.clone();

    let addr: SocketAddr = config
        .server
        .parse()
        .expect("Unable to parse socket address");

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    // Shared client connection for Reqwest
    let client = reqwest::Client::new();
    let routes = Router::new()
        .route("/health", get(get_health_endpoint))
        .route("/posts/:id", get(get_post_endpoint))
        .route("/posts", post(create_post_endpoint));

    let app = Router::new()
        .merge(routes)
        .with_state(client)
        .layer(TraceLayer::new_for_http());

    app
}
