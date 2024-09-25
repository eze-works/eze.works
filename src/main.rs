mod page;
mod post;

use anyhow::Context;
use post::{Metadata, Post};
use std::env;
use std::fs;
use std::sync::Arc;
use vintage::{status, Response, ServerConfig};

// Load the posts from the adjacent directory into memory
fn load_posts() -> anyhow::Result<Vec<Post>> {
    let mut posts = vec![];
    let posts_dir = env::current_dir()?.join("posts");
    let posts_iter = fs::read_dir(posts_dir)?;

    for file in posts_iter {
        let path = file?.path();
        let markdown = fs::read_to_string(&path)?;
        let (metadata, content) = markdown.split_once("+++").with_context(|| {
            format!("markdown file {} missing a metadata block", path.display())
        })?;
        let metadata: Metadata = serde_json::from_str(metadata)
            .with_context(|| format!("failed to parse metadata as json: {metadata}"))?;

        let basename = path.file_stem().unwrap();
        let slug = basename.to_string_lossy().to_string();

        let options = pulldown_cmark::Options::ENABLE_GFM;
        let parser = pulldown_cmark::Parser::new_ext(content, options);
        let mut html = String::new();
        pulldown_cmark::html::write_html_fmt(&mut html, parser)?;

        let post = Post {
            slug,
            content: html,
            metadata,
        };
        posts.push(post);
    }

    posts.sort_by_key(|p| p.metadata.date);
    posts.reverse();

    Ok(posts)
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Bundle in an `Arc` so the handlers can share it.
    let posts = Arc::new(load_posts()?);

    let config = ServerConfig::new()
        .on_get(["/", "/home"], {
            let posts = posts.clone();
            move |_, _| {
                let html = page::home(posts.as_slice()).to_string();
                Response::html(html)
            }
        })
        .on_get(["/post/{slug}", "/post/{slug}/"], {
            let posts = posts.clone();
            move |_, params| {
                let slug = &params["slug"];
                let Some(post) = posts.iter().find(|f| &f.slug == slug) else {
                    let html = page::not_found().to_string();
                    return Response::html(html).set_status(status::NOT_FOUND);
                };
                let html = page::single_post(post).to_string();
                Response::html(html)
            }
        })
        .unhandled(|_| {
            let html = page::not_found().to_string();
            Response::html(html).set_status(status::NOT_FOUND)
        })
        .serve_files("/assets", "./assets");

    let handle = vintage::start(config, "localhost:8000").unwrap();

    handle.join();
    Ok(())
}
