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

        let post = Post {
            slug,
            content: markdown_to_html(content)?,
            metadata,
        };
        posts.push(post);
    }

    posts.sort_by_key(|p| p.metadata.date);
    posts.reverse();

    Ok(posts)
}

fn markdown_to_html(markdown: &str) -> anyhow::Result<String> {
    use pulldown_cmark::html::write_html_fmt;
    use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, TextMergeStream};

    let mut events = Vec::new();
    let options = Options::ENABLE_GFM;
    let parser = TextMergeStream::new(Parser::new_ext(markdown, options));

    // The following mess takes care of adding anchor links to headings.
    // TODO: Figure out a way to clean it up
    let mut most_recent_heading = None;
    let mut heading_text: Option<String> = None;
    for mut event in parser {
        match &mut event {
            Event::Start(Tag::Heading { .. }) => {
                most_recent_heading = Some(events.len());
                events.push(event);
                events.push(Event::Html(CowStr::from("<a>")));
            }
            Event::End(TagEnd::Heading(_)) => {
                let Some(idx) = most_recent_heading.take() else {
                    panic!("unmatched heading tags!");
                };

                let text = heading_text.take().unwrap_or_default();

                let Some(Event::Start(Tag::Heading { id, .. })) = events.get_mut(idx) else {
                    unreachable!()
                };

                *id = Some(CowStr::from(text.clone()));

                let Some(Event::Html(html)) = events.get_mut(idx + 1) else {
                    unreachable!()
                };

                *html = CowStr::from(format!("<a href=\"#{}\">", text));

                events.push(Event::Html(CowStr::from("</a>")));
                events.push(event);
            }
            Event::Text(t) => {
                if most_recent_heading.is_some() {
                    heading_text = Some(t.to_ascii_lowercase().replace(' ', "-"));
                }
                events.push(event);
            }
            _ => {
                events.push(event);
            }
        };
    }

    let mut html = String::new();
    write_html_fmt(&mut html, events.into_iter())?;
    Ok(html)
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
