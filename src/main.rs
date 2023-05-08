use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;
// use std::net::SocketAddr;
// use tower_http::cors::{AllowedOrigins, CorsLayer};

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.
    let app: Router = Router::new().route("/", get(handler));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> Listening on localhost - port:  {}", addr.port());

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // axum:
}

async fn handler() -> impl IntoResponse {
    // Generate a random number between 1 and 100.
    let mut rng = thread_rng();
    let number = rng.gen_range(1..=100);

    // Return a HTML response.
    Html(format!(
        r#"
        <html>
            <head>
                <title>Guessing Game</title>
            </head>
            <body>
                <h1>Guess a number between 1 and 100</h1>
                <h2>{}</h2>
            </body>
        </html>
    "#,
        number
    ))
}
