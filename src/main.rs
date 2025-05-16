use axum::http::Response;
use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

mod app;
use crate::app::App;

mod api;
mod config;
mod domain_data_model;
mod rpc_data_model;

#[tokio::main]
async fn main() {
    // initialize tracing and logging
    tracing_subscriber::fmt::init();

    log::info!("App starting");

    // Setup rate limitter
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .error_handler(|error| match error {
                _ => Response::new("Wait at least 1 second.".into()),
            })
            .burst_size(5)
            .per_second(1)
            .finish()
            .unwrap(),
    );
    let governor_limiter = governor_conf.limiter().clone();
    // a separate background task to clean up
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(60));
            if governor_limiter.len() > 0 {
                log::info!("rate limiting storage size: {}", governor_limiter.len());
            }
            governor_limiter.retain_recent();
        }
    });

    let app = App::new();
    let bind_addr = app.get_bind_address();

    let routes = Router::new()
        .route("/", get(api::root))
        .route("/version", get(api::version))
        .route("/estimate", post(api::estimate))
        .layer(GovernorLayer {
            config: governor_conf,
        })
        .with_state(app);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    log::info!("App started on http://{}/", bind_addr);

    axum::serve(
        listener,
        routes.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
