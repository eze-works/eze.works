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

        let mut extension_options = comrak::ExtensionOptions::default();
        extension_options.footnotes = true;
        let options = comrak::Options {
            extension: extension_options,
            ..comrak::Options::default()
        };
        let html = comrak::markdown_to_html(content, &options);

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
        .get(["/", "/home"], {
            let posts = posts.clone();
            move |ctx, _| {
                let html = page::home(posts.as_slice()).to_string();
                ctx.with_status(200).with_html_body(html)
            }
        })
        .get(["/post/{slug}", "/post/{slug}/"], {
            let posts = posts.clone();
            move |ctx, params| {
                let slug = &params["slug"];
                let Some(post) = posts.iter().find(|f| &f.slug == slug) else {
                    let html = page::not_found().to_string();
                    return ctx.with_status(status::NOT_FOUND).with_html_body(html);
                };
                let html = page::single_post(post).to_string();
                ctx.with_status(status::OK).with_html_body(html)
            }
        })
        .not_found(|ctx| {
            let html = page::not_found().to_string();
            ctx.with_status(status::NOT_FOUND).with_html_body(html)
        });

    let pipeline = files.and(router);

    let server = start("localhost:8000", move |ctx| pipeline.run(ctx))?;

    server.join();
    Ok(())
}
