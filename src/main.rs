mod page;
mod post;

use anyhow::Context;
use post::{Metadata, Post};
use std::env;
use std::fs;

use pulldown_cmark::html::write_html_fmt;
use pulldown_cmark::{
    CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, TextMergeStream,
};
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

enum MarkdownState {
    InHeader(HeadingLevel, String),
    InCodeblock(String, String),
    Other,
}

fn process_markdown(markdown: &str) -> anyhow::Result<String> {
    let options = Options::ENABLE_GFM;
    let parser = TextMergeStream::new(Parser::new_ext(markdown, options));
    let mut events = Vec::new();
    let mut state = MarkdownState::Other;

    for event in parser {
        match &mut state {
            MarkdownState::Other => match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    state = MarkdownState::InHeader(level, String::new());
                }
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    let lang = if lang.is_empty() {
                        String::from("plaintext")
                    } else {
                        lang.to_string()
                    };
                    state = MarkdownState::InCodeblock(lang.to_string(), String::new());
                }
                Event::Code(code) => {
                    // Replace regular hyphens with unicode non-breaking hyphens
                    // https://stackoverflow.com/questions/8753296/how-to-prevent-line-break-at-hyphens-in-all-browsers
                    events.push(Event::Code(CowStr::from(code.replace('-', "‑"))));
                }
                e => {
                    events.push(e);
                }
            },
            MarkdownState::InHeader(level, contents) => match event {
                Event::Text(t) => {
                    *contents += &t;
                }
                Event::End(TagEnd::Heading(_)) => {
                    let id = contents.to_ascii_lowercase().replace(' ', "=");
                    let href = format!("#{}", &id);
                    let rewritten = html! {
                        custom [data_tagname: level.to_string(), id: id] {
                            a [href: href] {
                                text(contents);
                            }
                        }
                    }
                    .to_string();
                    events.push(Event::Html(CowStr::from(rewritten)));
                    state = MarkdownState::Other;
                }
                _ => panic!("unexpected event in heading: {:?}", event),
            },
            MarkdownState::InCodeblock(lang, contents) => match event {
                Event::Text(t) => {
                    *contents += &t;
                }
                Event::End(TagEnd::CodeBlock) => {
                    let data_lang = if lang == "plaintext" {
                        ""
                    } else {
                        lang.as_str()
                    };
                    let rewritten = html! {
                        pre [data_lang: data_lang] {
                            code [class: format!("language-{}", &lang)] {
                                text(contents);
                            }
                        }
                    }
                    .to_string();
                    events.push(Event::Html(CowStr::from(rewritten)));
                    state = MarkdownState::Other;
                }
                _ => panic!("unexpected event in codeblock: {:?}", event),
            },
        }
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
