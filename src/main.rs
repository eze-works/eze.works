mod page;
mod post;

use anyhow::Context;
use post::{Metadata, Post};
use std::env;
use std::fs;
use std::sync::Arc;
use vintage::pipe::{FileServer, Pipe, Router};
use vintage::{start, status};

// Load the posts from the adjacent directory into memory
fn load_posts() -> anyhow::Result<Vec<Post>> {
    let mut posts = vec![];
    let posts_dir = env::current_dir()?.join("posts");
    let posts_iter = fs::read_dir(posts_dir)?;
    let markdown_options = pulldown_cmark::Options::ENABLE_FOOTNOTES
        | pulldown_cmark::Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS;

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

        let parser = pulldown_cmark::Parser::new_ext(content, markdown_options);
        let mut html = String::new();
        pulldown_cmark::html::write_html_fmt(&mut html, parser)
            .with_context(|| format!("invalid markdown in file {path:?}"))?;

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

    let files = FileServer::new("/assets", "./assets");
    let router = Router::new()
        .get("/", {
            let posts = posts.clone();
            move |ctx, _| {
                let html = page::home(posts.as_slice()).to_string();
                ctx.with_status(200).with_html_body(html)
            }
        })
        .get("/post/{slug}", {
            let posts = posts.clone();
            move |ctx, params| {
                let slug = &params["slug"];
                let Some(post) = posts.iter().find(|f| &f.slug == slug) else {
                    return ctx.with_status(status::NOT_FOUND);
                };
                let html = page::single_post(post).to_string();
                ctx.with_status(status::OK).with_html_body(html)
            }
        });

    let pipeline = files.and(router);

    let server = start("localhost:8000", move |ctx| pipeline.run(ctx))?;

    server.join();
    Ok(())
}
