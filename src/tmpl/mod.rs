use crate::types::{Post, State};
use maud::{html, Markup, PreEscaped, DOCTYPE};
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
            " - "
            a href = ("/feeds") {"Feeds"}
        }
    }
}

fn blogpost_banner(post: &Post) -> Markup {
    html! {
        div class="blogpost-banner" {
            h1 class="title" { (post.frontmatter.title) }
            div class="taglist" {
                @for tag in post.frontmatter.tags.iter() {
                    a class="taglist-tag" href = (format!("/tag/{}", tag)) {(format!("#{} ", tag))}
                }
            }
            small class="time-to-read" { ({format!("Time to read: {}m", post.readtime)}) }
        }
    }
}

pub async fn render_blogpost(post: &Post) -> Markup {
    let content = html! {
        (blogpost_banner(post))
        div class="blogpost-body" {
            (maud::PreEscaped(post.rendered.clone()))
        }
    };
    base(Some(&post.frontmatter.title), content)
}

pub async fn render_postlist(state: Arc<State>) -> Markup {
    let content = render_list_of_posts(
        state.posts.read().await.to_vec(),
        "All Posts".to_string(),
        state.debug_mode,
    )
    .await;
    base(Some("All posts"), content)
}

pub async fn render_tagged_post_list(state: Arc<State>, tag: String) -> Result<Markup, Markup> {
    let filtered: Vec<Post> = state
        .posts
        .read()
        .await
        .iter()
        .cloned()
        .filter(|x| x.frontmatter.tags.contains(&tag))
        .collect();
    if filtered.is_empty() {
        let error_page = html! {
            p {(format!("No posts found with the tag #{}", tag))}
        };
        return Err(error_page);
    }
    let heading = format!("Posts tagged with #{}", tag);
    let body = render_list_of_posts(filtered, heading.clone(), state.debug_mode).await;
    let final_content = base(Some(&heading.to_string()), body);
    Ok(final_content)
}

pub async fn render_list_of_posts(posts: Vec<Post>, heading: String, debug: bool) -> Markup {
    let content = html! {
        h1{(heading)}
        ul class="post-list" {
            @for post in posts.iter() {
                @if post.frontmatter.published <= chrono::Utc::now().date_naive() && post.frontmatter.public {
                    li class = "post-link" {
                        span class="date" { {(post.frontmatter.published.format("Y%Y M%m D%d"))} " -- " }
                        a href = ({format!("/post/{}", post.frontmatter.slug)}) {(post.frontmatter.title)}
                    }
                } @else if std::env::var("SITE_DEBUG").is_ok() || debug {
                    li class = "post-link" {
                        span class="date" { "UNPUBLISHED -- " }
                        a href = ({format!("/post/{}", post.frontmatter.slug)}) {(post.frontmatter.title)}
                    }
                }
            }
        }
    };
    content
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
            li{"Rust, Nix, Elixir, Haskell and Other languages."}
            li{"Docker, Linux, k8s, gRPC, MQTT"}
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
            li{"I Know too much about rhythm games."}
            li{"Outspokenly Autistic and Proudly Trans."}
        }
    };
    base(Some("About Me"), content)
}

pub async fn render_feeds() -> Markup {
    let content = html! {
        h1{"Feeds"}
        ul {
            li{a href="https://whydoesntmycode.work/feeds/atom.xml" {"Atom" }};
        }
    };
    base(Some("Feeds"), content)
}
