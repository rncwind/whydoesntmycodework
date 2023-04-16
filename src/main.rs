mod handlers;
mod tmpl;
mod types;

use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use tracing::*;
use tui::{backend::CrosstermBackend, Terminal};
use types::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Getting host info from Dhall config");
    let host: HostSettings = serde_dhall::from_file("./host.dhall").parse().unwrap();
    info!(
        "Will set up host on {} ({})",
        host.hostname,
        host.host_string()
    );

    info!("Getting site settings");
    let ss = SiteSettings::default();

    info!("Init state");
    let state: Arc<State> = Arc::new(State::new(ss));

    info!("Creating router");
    let app = Router::new()
        .route("/", get(handlers::list_posts))
        .route("/post/:slug", get(handlers::blogpost))
        .layer(Extension(state))
        .fallback(handlers::handle_404);

    info!("Serving!");
    axum::Server::bind(&host.host_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
