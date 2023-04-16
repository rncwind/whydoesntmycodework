mod tmpl;
mod types;

use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    response::Html,
    routing::get,
    Extension, Router,
};
use maud::{html, Markup};
use tracing::*;

use types::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Working directory is {:?}", std::env::current_dir());

    info!("Getting host info from Dhall config");
    let host: HostSettings = serde_dhall::from_file("./host.dhall").parse().unwrap();
    info!(
        "Will set up host on {} ({})",
        host.hostname,
        host.host_string()
    );

    info!("App starting up!");
    // Build our application as a router with a single route.

    info!("Getting site settings");
    let ss = SiteSettings::default();

    info!("Init state");
    let state: Arc<State> = Arc::new(State::new(ss));

    info!("Creating router");

    let app = Router::new()
        .route("/", get(tmpl::list_posts))
        .route("/post/:slug", get(tmpl::blogpost))
        .layer(Extension(state))
        .fallback(tmpl::handle_404);

    info!("Serving!");
    axum::Server::bind(&host.host_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
