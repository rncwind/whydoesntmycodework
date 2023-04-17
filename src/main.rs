mod handlers;
mod tmpl;
mod types;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    routing::get,
    BoxError, Extension, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use tracing::*;
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

    info!("Creating rustls config");
    info!("Reading certs from {:?}", host.cert_path);
    let config = RustlsConfig::from_pem_file(
        host.cert_path.join("cert.pem"),
        host.cert_path.join("key.pem"),
    )
    .await
    .unwrap();

    info!("Getting site settings");
    let ss = SiteSettings::default();

    info!("Init state");
    let state: Arc<State> = Arc::new(State::new(ss));

    info!("Spawning HTTP redirector");
    tokio::spawn(redirect_http_to_https(host.ports, host.ip_addr));

    info!("Creating router");
    let app = Router::new()
        .route("/", get(handlers::home))
        .route("/blog", get(handlers::list_posts))
        .route("/post/:slug", get(handlers::blogpost))
        .layer(Extension(state))
        .fallback(handlers::handle_404);

    info!("Serving!");
    let addr: SocketAddr = host.host_string().parse().unwrap();
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn redirect_http_to_https(ports: Ports, ip: std::net::IpAddr) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::new(ip, ports.http as u16);
    tracing::debug!("http redirect listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(redirect.into_make_service())
        .await
        .unwrap();
}
