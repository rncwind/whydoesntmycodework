use crate::tmpl::{render_about, render_blogpost, render_feeds, render_home, render_postlist};
use crate::types::State;

use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::Path, headers::ContentType, http::StatusCode, Extension, TypedHeader};
use hyper::HeaderMap;
use lazy_static::lazy_static;
use maud::{html, Markup};
use prometheus::{opts, register_int_counter_vec, Encoder, IntCounterVec, TextEncoder};
use serde::Deserialize;
use std::sync::Arc;

lazy_static! {
    static ref BLOGPOST_HIT_COUNTER: IntCounterVec = register_int_counter_vec!(
        opts!("blogpost_hits", "Number of hits to blogposts"),
        &["name"]
    )
    .unwrap();
    static ref PAGE_HIT_COUNTER: IntCounterVec =
        register_int_counter_vec!(opts!("page_hits", "Non-Blogpost page hits"), &["name"]).unwrap();
    static ref FEED_HIT_COUNTER: IntCounterVec =
        register_int_counter_vec!(opts!("feed_hits", "Feed Hits"), &["name"]).unwrap();
}

#[derive(Deserialize)]
pub struct AdminToken {
    admin_token: String,
}

pub async fn list_posts(Extension(state): Extension<Arc<State>>) -> Markup {
    PAGE_HIT_COUNTER.with_label_values(&["post_list"]).inc();
    render_postlist(state).await
}

pub async fn blogpost(
    Path(slug): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> (StatusCode, Markup) {
    for post in state.posts.read().await.iter() {
        if post.frontmatter.slug == slug {
            BLOGPOST_HIT_COUNTER
                .with_label_values(&[post.frontmatter.title.clone().as_str()])
                .inc();
            return (StatusCode::OK, render_blogpost(post).await);
        }
    }
    handle_404().await
}

pub async fn home() -> Markup {
    PAGE_HIT_COUNTER.with_label_values(&["home"]).inc();
    render_home().await
}

pub async fn handle_404() -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, html! {h1{"Move Along"}})
}

pub async fn about() -> Markup {
    PAGE_HIT_COUNTER.with_label_values(&["about"]).inc();
    render_about().await
}

pub async fn generate_atom_feed(Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    FEED_HIT_COUNTER.with_label_values(&["atom"]).inc();
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
        (
            StatusCode::FORBIDDEN,
            html! {"Nothing To See Here! (For you anyway)"},
        )
    }
}

pub async fn feeds() -> Markup {
    PAGE_HIT_COUNTER.with_label_values(&["feeds"]).inc();
    render_feeds().await
}
