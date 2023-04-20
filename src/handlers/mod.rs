use crate::tmpl::{render_about, render_blogpost, render_home, render_postlist};
use crate::types::State;
use axum::{extract::Path, http::StatusCode, Extension};
use maud::{html, Markup};
use std::sync::Arc;

pub async fn list_posts(Extension(state): Extension<Arc<State>>) -> Markup {
    render_postlist(state).await
}

pub async fn blogpost(
    Path(slug): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> (StatusCode, Markup) {
    for post in &state.posts {
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
