mod domainsocket;
mod handlers;
mod tmpl;
mod types;

use axum::body;
use prometheus::{Encoder, TextEncoder};
use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::header::{self, HeaderValue, CONTENT_TYPE},
    http::{StatusCode, Uri},
    response::Redirect,
    response::Response,
    routing::{get, post},
    BoxError, Extension, Router, ServiceExt,
};
use std::io::Write;
use tokio::net::UnixListener;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tracing::*;
use types::*;

use crate::domainsocket::ServerAccept;

fn create_admin_token() -> String {
    match std::env::var("ADMIN_TOKEN") {
        Ok(p) => {
            info!("Got admin token from envvar: {}", p);
            p
        }
        Err(why) => {
            error!("Couldn't get admin token {why:?}");
            let admin_token: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();
            info!("Admin key is {}", admin_token);
            let mut dir = std::env::temp_dir();
            dir.push("admin_token");
            let mut admintokenfile = File::create(dir).expect("Couldn't make admin token file");
            info!("Admin key written to {:?}", admintokenfile);
            admintokenfile
                .write_all(admin_token.as_bytes())
                .expect("Couldn't write admin token to file");
            admin_token
        }
    }
}

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

    info!("Generating new admin token");
    let admin_token = create_admin_token();

    info!("Init state");
    let state: Arc<State> = Arc::new(State::new(ss, admin_token));

    info!("Setting up static file service");
    let staticfiles = ServeDir::new("static");

    let middleware = tower::ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::appending(
            header::HeaderName::from_static("x-clacks-overhead"),
            Some(header::HeaderValue::from_static(
                "GNU Terry Pratchett, Akira Complex, Natalie Nguyen, Brianna Ghey",
            )),
        ))
        .layer(SetResponseHeaderLayer::appending(
            header::HeaderName::from_static("x-powered-by"),
            Some(header::HeaderValue::from_static(
                "Coffee, Estradiol, Anger and Rust",
            )),
        ))
        .layer(Extension(state));

    info!("Creating router");
    let app = Router::new()
        .route("/", get(handlers::home))
        .route("/blog", get(handlers::list_posts))
        .route("/about", get(handlers::about))
        .route("/post/:slug", get(handlers::blogpost))
        .route("/feeds", get(handlers::feeds))
        .route("/feeds/atom.xml", get(handlers::generate_atom_feed))
        .route("/api/admin/reload", post(handlers::reload_posts))
        .route("/stats", get(stats))
        .nest_service("/static", staticfiles)
        .layer(middleware)
        .fallback(handlers::handle_404);

    match std::env::var("SOCKET_PATH") {
        // Production will use this. We bind on a unix socket and then proxy it
        // with Nginx for ease of ACME on Nix.
        Ok(p) => {
            info!("Found a socket on SOCKET_PATH: {:?}", p);
            // Clean up leftovers from previous runs
            let _ = std::fs::remove_file(&p);
            // Bind to our unix socket.
            info!("Binding to unix socket");
            let unixsock = UnixListener::bind(&p).unwrap();
            let perm = std::fs::Permissions::from_mode(0o777);
            std::fs::set_permissions(p, perm).unwrap();
            // And serve the server.
            info!("Serving!");
            axum::Server::builder(ServerAccept { uds: unixsock })
                .serve(app.into_make_service_with_connect_info::<domainsocket::UdsConnectInfo>())
                .await
                .unwrap();
        }
        Err(e) => {
            // Dev case!
            warn!("Couldn't get a unix socket, trying to serve normally. Are we on dev?");
            warn!("Error was: {}", e);
            let addr: SocketAddr = host.host_string().parse().unwrap();
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap()
        }
    }
}

async fn stats() -> Response {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(body::boxed(body::Full::from(buffer)))
        .unwrap()
}
