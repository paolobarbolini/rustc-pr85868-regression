use std::sync::Arc;

use axum::handler::{get, post};
use axum::http::StatusCode;
use axum::AddExtensionLayer;
use axum::Router;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();

    let route1 = Router::new()
        .route(
            "/1",
            get(|| async move { StatusCode::OK }).post(|| async move { StatusCode::OK }),
        )
        .route("/2", post(|| async move { StatusCode::OK }));
    let route2 = Router::new()
        .route(
            "/3",
            get(|| async move { StatusCode::OK }).post(|| async move { StatusCode::OK }),
        )
        .route("/4", get(|| async move { StatusCode::OK }))
        .route("/5", post(|| async move { StatusCode::OK }))
        .route("/6", post(|| async move { StatusCode::OK }))
        .route("/7", get(|| async move { StatusCode::OK }));
    let route3 = Router::new()
        .route("/8", post(|| async move { StatusCode::OK }))
        .route("/9", get(|| async move { StatusCode::OK }));
    let route4 = Router::new().route("/10", post(|| async move { StatusCode::OK }));
    let route5 = Router::new().route("/11", post(|| async move { StatusCode::OK }));
    let route6 = Router::new()
        .route(
            "/12",
            get(|| async move { StatusCode::OK }).post(|| async move { StatusCode::OK }),
        )
        .route(
            "/13",
            post(|| async move { StatusCode::OK }).delete(|| async move { StatusCode::OK }),
        );

    let api = Router::new()
        .nest("/14", route1)
        .nest("/15", route4)
        .nest("/16", route2)
        .nest("/17", route3)
        .nest("/18", route5)
        .nest("/19", route6);
    let router = Router::new()
        .nest("/20", api)
        .route("/21", get(|| async move { StatusCode::OK }));

    let app = router
        .layer(AddExtensionLayer::new(Arc::new(1234)))
        .layer(TraceLayer::new_for_http());

    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
