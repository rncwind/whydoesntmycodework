use crate::types::State;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension};
use maud::{html, Markup};
use std::sync::Arc;

pub async fn list_posts(Extension(state): Extension<Arc<State>>) -> Markup {
    html! {
        // List of all blog posts.
        ul {
            @for post in &state.posts {
                li {
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
            return (
                StatusCode::OK,
                html! (
                    h3 { ({format!("Time to read: {}m", post.readtime)}) }
                    (maud::PreEscaped(post.rendered.clone()))
                ),
            );
        }
    }
    handle_404().await
}

pub async fn handle_404() -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, html! {h1{"Move Along"}})
}
