use crate::tmpl::{render_about, render_blogpost, render_feeds, render_home, render_postlist};
use crate::types::State;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::Path, headers::ContentType, http::StatusCode, Extension, TypedHeader};
use hyper::HeaderMap;
use maud::{html, Markup};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct AdminToken {
    admin_token: String,
}

pub async fn list_posts(Extension(state): Extension<Arc<State>>) -> Markup {
    render_postlist(state).await
}

pub async fn blogpost(
    Path(slug): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> (StatusCode, Markup) {
    for post in state.posts.read().await.iter() {
        if post.frontmatter.slug == slug {
            return (StatusCode::OK, render_blogpost(post).await);
        }
    }
    handle_404().await
}

pub async fn home() -> Markup {
    render_home().await
}

pub async fn handle_404() -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, html! {h1{"Move Along"}})
}

pub async fn about() -> Markup {
    render_about().await
}

pub async fn generate_atom_feed(Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    // Atom has it's own MIME type, we should use it.
    headers.insert("content-type", "application/atom+xml".parse().unwrap());
    (headers, (*state.atom_feed.read().await).to_string())
}

pub async fn reload_posts(
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<AdminToken>,
) -> (StatusCode, Markup) {
    if payload.admin_token == state.admin_token {
        let newposts = state.generate_posts();
        *state.posts.write().await = newposts;
        let new_feed = state.generate_atom_feed().await;
        *state.atom_feed.write().await = new_feed;
        (StatusCode::OK, html! {"Refreshed!"})
    } else {
        (StatusCode::FORBIDDEN, html! {"Bugger off!"})
    }
}

pub async fn feeds() -> Markup {
    render_feeds().await
}
