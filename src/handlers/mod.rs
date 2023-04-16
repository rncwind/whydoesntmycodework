use crate::tmpl::render_blogpost;
use crate::types::{Post, State};
use axum::{extract::Path, http::StatusCode, Extension};
use maud::{html, Markup, DOCTYPE};
use std::sync::Arc;

pub async fn list_posts(Extension(state): Extension<Arc<State>>) -> Markup {
    html! {
        ul class="post-list" {
            @for post in &state.posts {
                li class = "post-link" {
                    a href = ({format!("/post/{}", post.frontmatter.slug)}) {(post.frontmatter.title)}
                }
            }
        }
    }
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

pub async fn handle_404() -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, html! {h1{"Move Along"}})
}
