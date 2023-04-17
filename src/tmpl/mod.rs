use crate::types::{Post, State};
use maud::{html, Markup, DOCTYPE};
use std::sync::Arc;

// Eventually everything reaches this. This is our base template.
// We keep everything nice and consistent by puting all our CSS and
// stuff into here.
fn base(title: Option<&str>, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang = "en" {
            head {
                meta charset="utf-8";
                title {
                    @if let Some(title) = title {
                        (title) " - Why Doesnt My Code Work?"
                    } @else {
                        "Why Doesnt My Code Work?"
                    }
                }
            }
            (navbar())
            (content)
        }
    }
}

fn navbar() -> Markup {
    html! {
        nav {
            a href = ("/") {"Home"}
            {"|"}
            a href = ("/blog") {"Blog"}
        }
    }
}

pub async fn render_blogpost(post: &Post) -> Markup {
    let content = html! {
        div class="blogpost-banner" {
            h1 class="title" { (post.frontmatter.title) }
            small class="time-to-read" { ({format!("Time to read: {}m", post.readtime)}) }
        }
        div class="blogpost-body" {
            (maud::PreEscaped(post.rendered.clone()))
        }
    };
    base(Some(&post.frontmatter.title), content)
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
    let content = html! {
        p{"Ain’t Nobody Here But Us Chickens"}
        img src="https://web.archive.org/web/20091027035606im_/http://es.geocities.com/melgarbeatles6/barraconstruction.gif" alt="Geocities Under Construction Gif";
    };
    base(None, content)
}
