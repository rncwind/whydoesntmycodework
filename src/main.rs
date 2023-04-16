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
        .route("/", get(list_posts))
        .route("/post/:slug", get(view_post))
        // .route("/path/:user_id", get(path))
        .layer(Extension(state));

    info!("Serving!");
    axum::Server::bind(&host.host_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn view_post(
    Path(slug): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> maud::Markup {
    for post in &state.posts {
        if post.frontmatter.slug == slug {
            return html!((maud::PreEscaped(post.rendered.clone())));
        }
    }
    html!(h1{"404 lol"})
}

async fn list_posts(Extension(state): Extension<Arc<State>>) -> Markup {
    html! {
        ul {
            @for post in &state.posts {
                li {
                    a href = ({format!("/post/{}", post.frontmatter.slug)}) {(post.frontmatter.title)}
                }
            }
        }
    }
}
