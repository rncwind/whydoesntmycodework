use crate::types::{Post, State};
use axum::{extract::Path, http::StatusCode, Extension};
use maud::{html, Markup, DOCTYPE};
use std::sync::Arc;

fn post_header(post: &Post) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title{ (post.frontmatter.title) }
    }
}

fn post_banner(post: &Post) -> Markup {
    html! {
        ul class="navigation" {
            li class = "nav-element"{  }
        }
        br{}
        h1 class="title" { (post.frontmatter.title) }
        h3 class="time-to-read" { ({format!("Time to read: {}m", post.readtime)}) }
    }
}

pub async fn render_blogpost(post: &Post) -> Markup {
    html! {
        (post_header(post))
        (post_banner(post))
        (maud::PreEscaped(post.rendered.clone()))
    }
}
