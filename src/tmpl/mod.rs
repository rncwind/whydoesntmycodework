use crate::types::{Post, State};
use maud::{html, Markup, DOCTYPE};
use std::sync::Arc;

fn navbar() -> Markup {
    html! {
        ul class = "navigation" {
            li class = "nav-element" {
                a href = ("/") {"Home"}
            }
            li class = "nav-element" {
                a href = ("/blog") {"Blog"}
            }
        }
    }
}

fn standard_header() -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
    }
}

fn post_html_header(post: &Post) -> Markup {
    html! {
        (standard_header())
        title{ (post.frontmatter.title) }
    }
}

fn post_banner(post: &Post) -> Markup {
    html! {
        (navbar())
        h1 class="title" { (post.frontmatter.title) }
        h3 class="time-to-read" { ({format!("Time to read: {}m", post.readtime)}) }
    }
}

pub async fn render_blogpost(post: &Post) -> Markup {
    html! {
        (post_html_header(post))
        (post_banner(post))
        (maud::PreEscaped(post.rendered.clone()))
    }
}

pub async fn render_postlist(state: Arc<State>) -> Markup {
    html! {
        (navbar())
        ul class="post-list" {
            @for post in &state.posts {
                li class = "post-link" {
                    a href = ({format!("/post/{}", post.frontmatter.slug)}) {(post.frontmatter.title)}
                }
            }
        }
    }
}

pub async fn render_home() -> Markup {
    html! {
        (navbar())
        p{"Ain’t Nobody Here But Us Chickens"}
        img src="https://web.archive.org/web/20091027035606im_/http://es.geocities.com/melgarbeatles6/barraconstruction.gif" alt="Geocities Under Construction Gif";
    }
}
