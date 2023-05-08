use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
// use std::net::SocketAddr;
// use tower_http::cors::{AllowedOrigins, CorsLayer};

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // axum:
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
