use chrono::{NaiveDate, NaiveDateTime};
use std::cmp::Ordering;
use std::path::PathBuf;
use tokio::sync::RwLock;

use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use serde::Deserialize;
use thiserror::Error;
use tracing::*;

const POST_BASE: &str = "https://whydoesntmycode.work/post/";

#[derive(Debug)]
pub struct SiteSettings {
    pub posts_path: PathBuf,
}

impl Default for SiteSettings {
    fn default() -> Self {
        Self {
            posts_path: "./posts/".parse().unwrap(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Ports {
    pub http: u32,
    pub https: u32,
}

#[derive(Deserialize, Debug)]
pub struct HostSettings {
    pub hostname: String,
    pub ip_addr: std::net::IpAddr,
    pub ports: Ports,
}

impl HostSettings {
    pub fn host_string(&self) -> String {
        format!("{}:{}", self.ip_addr, self.ports.https)
    }
}

#[derive(Error, Debug)]
pub enum PostParseError {
    #[error("Frontmatter for a post was invalid")]
    FrontmatterError,
    #[error("There is no front matter!")]
    NoFrontmatter,
    #[error("Only found one ---, yaml is probably unterminated!")]
    UnterminatedFrontmatter,
}

#[derive(PartialEq, Clone)]
pub struct Post {
    pub frontmatter: FrontMatter,
    pub rendered: String,
    pub readtime: u64,
}

impl Post {
    fn new(
        content: String,
        co: &ComrakOptions,
        cplug: &ComrakPlugins,
    ) -> Result<Post, PostParseError> {
        let frontmatter = FrontMatter::new(&content)?;
        //let rendered = markdown_to_html(&content, co);
        let rendered = markdown_to_html_with_plugins(&content, co, cplug);
        let readtime = estimated_read_time::text(
            &content,
            &estimated_read_time::Options::new()
                .technical_document(true)
                .technical_difficulty(2)
                .build()
                .unwrap_or_default(),
        )
        .seconds()
            / 60;
        Ok(Post {
            frontmatter,
            rendered,
            readtime,
        })
    }

    fn get_full_url(&self) -> String {
        format!("{}{}", POST_BASE, self.frontmatter.slug)
    }

    fn as_atom(&self) -> String {
        format!(
            "
<entry>
    <id>{}</id>
    <title>{}</title>
    <published>{}</published>
    <updated>{}</updated>
    <content type=\"html\" xml:base=\"{}\"><!CDATA[{}]> </content>
    <link href=\"{}\" rel=\"alternate\" />
</entry>",
            self.get_full_url(),
            self.frontmatter.title,
            self.frontmatter.published,
            self.frontmatter.updated.unwrap_or_default(),
            self.get_full_url(),
            self.rendered,
            self.get_full_url()
        )
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.frontmatter.published.cmp(&other.frontmatter.published))
    }
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct FrontMatter {
    pub title: String,
    pub slug: String,
    pub published: chrono::NaiveDate,
    pub updated: Option<chrono::NaiveDate>,
    pub tags: Vec<String>,
}

impl FrontMatter {
    fn new(content: &str) -> Result<FrontMatter, PostParseError> {
        let matches: Vec<_> = content.match_indices("---").collect();
        if matches.is_empty() {
            Err(PostParseError::NoFrontmatter)
        } else if matches.len() == 1 {
            Err(PostParseError::UnterminatedFrontmatter)
        } else {
            let start = (matches[0].0) + 3; // Skip over the first 3 ---
            let end = matches[1].0;
            let slice = &content[start..end].to_string();
            info!("{}", slice);
            match serde_yaml::from_str(slice) {
                Ok(x) => Ok(x),
                Err(e) => {
                    error!("{}", e);
                    Err(PostParseError::FrontmatterError)
                }
            }
        }
    }
}

pub struct State {
    pub posts: RwLock<Vec<Post>>,
    pub admin_token: String,
    pub atom_feed: Option<String>,
    posts_path: PathBuf,
}

impl State {
    fn get_posts(
        post_dir: Option<PathBuf>,
        comrak_opts: &ComrakOptions,
        comrak_plugins: &ComrakPlugins,
    ) -> Vec<Post> {
        let mut v: Vec<Post> = Vec::new();
        let p: PathBuf = post_dir.unwrap_or("./posts/".parse().unwrap());
        let post_paths = std::fs::read_dir(p).unwrap();
        for path in post_paths {
            let validpath = match path {
                Ok(p) => p.path(),
                Err(x) => {
                    error!("Could not get valid path from file in posts dir. {}", x);
                    continue;
                }
            };

            // Now we have a valid file path that we can read the markdown from.
            let filename = String::from(validpath.file_stem().unwrap().to_str().unwrap());
            println!("{:?}", validpath);
            let content = std::fs::read_to_string(validpath).unwrap();
            let post = Post::new(content, comrak_opts, comrak_plugins);
            match post {
                Ok(post) => {
                    if post.frontmatter.published <= chrono::Utc::now().date_naive() {
                        v.push(post)
                    } else {
                        info!(
                            "Post {} isn't due to be published yet. Skipping",
                            post.frontmatter.title
                        );
                    }
                }
                Err(e) => {
                    warn!("{:?} on file {:?}, SKIPPING", e, filename);
                    continue;
                }
            }
        }
        v
    }

    pub fn new(settings: SiteSettings, admin_token: String) -> Self {
        let adapter = SyntectAdapter::new("base16-eighties.dark");
        let mut comrak_opts = ComrakOptions::default();
        comrak_opts.extension.front_matter_delimiter = Some("---".to_owned());
        let mut comrak_plugins = ComrakPlugins::default();
        comrak_plugins.render.codefence_syntax_highlighter = Some(&adapter);
        let mut posts = State::get_posts(
            Some(settings.posts_path.clone()),
            &comrak_opts,
            &comrak_plugins,
        );
        posts.sort_by(|a, b| b.frontmatter.published.cmp(&a.frontmatter.published));
        Self {
            posts: RwLock::new(posts),
            posts_path: settings.posts_path,
            atom_feed: None,
            admin_token,
        }
    }

    pub fn generate_posts(&self) -> Vec<Post> {
        let adapter = SyntectAdapter::new("base16-eighties.dark");
        let mut comrak_opts = ComrakOptions::default();
        comrak_opts.extension.front_matter_delimiter = Some("---".to_owned());
        let mut comrak_plugins = ComrakPlugins::default();
        comrak_plugins.render.codefence_syntax_highlighter = Some(&adapter);
        let mut posts =
            State::get_posts(Some(self.posts_path.clone()), &comrak_opts, &comrak_plugins);
        posts.sort_by(|a, b| b.frontmatter.published.cmp(&a.frontmatter.published));
        posts
    }

    pub async fn generate_atom_feed(&self) -> String {
        //let rss_entries: Vec<String> = Vec::new();
        let atom_header = "
<?xml version='1.0' encoding='UTF-8'?>
<feed xmlns=\"http://www.w3.org/2005/Atom\">
    <id>https://whydoesntmycode.work/blog.atom</id>
    <title>Why Doesn't My Code Work?</title>
    <author>
        <name>Freyja</name>
        <email>rncwnd@whydoesntmycode.work</email>
    </author>
    <link href=\"https://whydoesntmycode.work/blog.atom\" rel=\"self\" />
    <generator uri=\"https://whydoesntmycode.work\" version=\"3.0.0\">whydoesntmycode.work</generator>
";
        let mut feed = format!("{}", atom_header);
        let rss_entries: Vec<String> = self
            .posts
            .read()
            .await
            .iter()
            .map(|x| x.as_atom())
            .collect();
        for entry in rss_entries {
            feed = format!("{}{}", feed, entry)
        }
        format!("{}\n</feed>", feed)
    }
}
