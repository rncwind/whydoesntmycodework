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
                link rel="stylesheet" href={"/static/css/debugdance.css"};
                link rel="stylesheet" href={"https://fonts.googleapis.com/css2?family=Fira+Code&family=Fira+Sans&display=swap"};
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
            " - "
            a href = ("/blog") {"Blog"}
            " - "
            a href = ("/about") {"About Me"}
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
    let content = html! {
        h1{"All Posts"}
        ul class="post-list" {
            @for post in &state.posts {
                li class = "post-link" {
                    span class="date" { {(post.frontmatter.published.format("Y%Y M%m D%d"))} " -- " }
                    a href = ({format!("/post/{}", post.frontmatter.slug)}) {(post.frontmatter.title)}
                }
            }
        }
    };
    base(Some("All posts"), content)
}

pub async fn render_home() -> Markup {
    let content = html! {
        h1{"Why Doesn't My Code Work?"}
        p{"Sorry, the blog title is clickbait, glad it worked though!"};
        p{"I'm Freyja, by the way."}
        p{"Whilst I cant help you with your code, perhaps you might find a technical blog
           that covers weird parts of computing that I stuble across interesting? If so check the " a href="/blog" {"Blog"} " section"}
        p{"If you want to know more about me, and my skillset, head over to " a href="/about" {"about me"} " section"}
        p{"If you did actually want help with your code, check out this " a href="https://stackoverflow.com/" {"crazy new website" }}
    };
    base(None, content)
}

pub async fn render_about() -> Markup {
    let content = html! {
        h1{"Freyja"}
        h2{"Skills"}
        ul {
            li{"Rust, Nix, Haskell and Other languages."}
            li{"Docker, Linux, k8s"}
            li{"Machine Learning, Data Science, Programming Language Theory"}
        }

        h2{"Projects"}
        ul {
            li{a href="https://github.com/rncwind/whydoesntmycodework" {"Site"} ": The site you are currently reading"};
            li{a href="https://github.com/rncwind/Daemonium" {"daemonium"} ": An Esoteric Programming Language to summon Daemons"};
            li{a href="https://github.com/rncwind/sktime-neuro" {"sktime-neuro"} ": Subject of my masters, time series machine learning for neurological data"};
            li{a href="https://github.com/rncwind/rsrs" {"rsrs"} ": a simple reverse-shell generator for CTFs"};
        }

        h2{"Other Stuff About Me"}
        ul {
            li{"I'm a Forever DM. Right now I play a lot of Pathfinder 2e"}
            li{"I like shooting arrows out of bows"}
            li{"Outspokenly Autistic and Proudly Trans."}
        }
    };
    base(Some("About Me"), content)
}
