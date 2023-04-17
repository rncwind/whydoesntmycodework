use std::path::PathBuf;

use comrak::{markdown_to_html, ComrakOptions};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use thiserror::Error;
use tracing::*;

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
    pub cert_path: PathBuf,
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
}

pub struct Post {
    pub frontmatter: FrontMatter,
    pub rendered: String,
    pub readtime: u64,
}

#[derive(Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub slug: String,
    pub date: Option<String>,
}

impl Post {
    fn new(content: String, co: &ComrakOptions) -> Result<Post, PostParseError> {
        let frontmatter = FrontMatter::new(&content)?;
        let rendered = markdown_to_html(&content, co);
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
}

impl FrontMatter {
    fn new(content: &str) -> Result<FrontMatter, PostParseError> {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(content);
        let fm = match result.data {
            Some(fm) => fm,
            None => return Err(PostParseError::NoFrontmatter),
        };
        //result.data.unwrap().deserialize().unwrap()
        match fm.deserialize() {
            Ok(x) => Ok(x),
            Err(y) => {
                warn!("{:?}", y);
                Err(PostParseError::FrontmatterError)
            }
        }
    }
}

pub struct State {
    pub posts: Vec<Post>,
}

impl State {
    fn get_posts(post_dir: Option<PathBuf>, comrak_opts: &ComrakOptions) -> Vec<Post> {
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
            let content = std::fs::read_to_string(validpath).unwrap();
            let post = Post::new(content, comrak_opts);
            match post {
                Ok(post) => v.push(post),
                Err(e) => {
                    warn!("{:?} on file {:?}, SKIPPING", e, filename);
                    continue;
                }
            }
        }
        v
    }

    pub fn new(settings: SiteSettings) -> Self {
        let mut comrak_opts = ComrakOptions::default();
        comrak_opts.extension.front_matter_delimiter = Some("---".to_owned());
        Self {
            posts: State::get_posts(Some(settings.posts_path), &comrak_opts),
        }
    }
}
