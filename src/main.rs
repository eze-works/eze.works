mod page;
mod post;

use anyhow::Context;
use post::{Metadata, Post};
use std::env;
use std::fs;

use pulldown_cmark::html::write_html_fmt;
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, TextMergeStream};
use std::sync::Arc;
use toph::{html, text};
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
            content: process_markdown(content)?,
            metadata,
        };
        posts.push(post);
    }

    posts.sort_by_key(|p| p.metadata.date);
    posts.reverse();

    Ok(posts)
}

enum EventMarker {
    Heading { start: usize, end: usize },
    Other,
}

fn process_markdown(markdown: &str) -> anyhow::Result<String> {
    let mut events = Vec::new();
    let mut markers = Vec::new();

    let options = Options::ENABLE_GFM;
    let parser = TextMergeStream::new(Parser::new_ext(markdown, options));

    let mut last_seen_heading = 0;

    for (index, event) in parser.into_iter().enumerate() {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                last_seen_heading = index;
            }
            Event::End(TagEnd::Heading(_)) => {
                markers.push(EventMarker::Heading {
                    start: last_seen_heading,
                    end: index,
                });
            }
            _ => {}
        };

        events.push(event);
    }

    insert_heading_anchor_links(&mut events, markers.as_ref());
    let mut html = String::new();
    write_html_fmt(&mut html, events.into_iter())?;
    Ok(html)
}

fn insert_heading_anchor_links(events: &mut Vec<Event>, markers: &[EventMarker]) {
    for marker in markers {
        let EventMarker::Heading { start, end } = marker else {
            continue;
        };

        let Some(heading_events) = events.get((start + 1)..*end) else {
            panic!("invalid event markers!");
        };

        let mut heading_text = String::new();
        for event in heading_events {
            if let Event::Text(t) = event {
                heading_text += t.as_ref();
            }
        }
        let normalized = heading_text.replace(' ', "-").to_ascii_lowercase();
        let anchor = html! {
            a[href: format!("#{}", &normalized)] {
                text(heading_text);
            }
        };
        let anchor = Event::Html(CowStr::from(anchor.to_string()));

        // Replaces the heading text with the anchor html
        events.splice((start + 1)..*end, [anchor]);

        // Updates the heading id
        let Some(Event::Start(Tag::Heading { id, .. })) = events.get_mut(*start) else {
            unreachable!();
        };

        *id = Some(CowStr::from(normalized));
    }
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
