mod home;
mod not_found;
mod single_post;

pub use home::home;
pub use not_found::not_found;
pub use single_post::single_post;

use toph::{html, text, Node};

#[derive(Default, Clone)]
struct BaseLayoutOptions {
    title: String,
}

fn base_layout(content: Node, opts: BaseLayoutOptions) -> Node {
    html! {
        doctype [html: true] {}
        html {
            head {
                metadata(opts);
                link[rel: "stylesheet", href: "/assets/css/reset.css"] {}
                link[rel: "stylesheet", href: "/assets/css/fonts.css"] {}
                link[rel: "stylesheet", href: "/assets/css/styles.css"] {}

            }
            body {
                div[id: "main-content"] {
                    div[id: "logo", class: "center"] {
                        a[href: "/"] {
                            text("e.w");
                        }
                    }
                    content;
                }
                footer();
            }
        }

    }
}

fn metadata(opts: BaseLayoutOptions) -> Node {
    let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
    let title = if opts.title.is_empty() {
        "Eze Works".to_string()
    } else {
        format!("{} | Eze Works", opts.title)
    };
    html! {
        meta[charset: "utf-8"]{}
        meta[name: "viewport", content: viewport]{}
        title {
            text(title);
        }
    }
}

fn footer() -> Node {
    html! {
        footer {
            p {
                text("This site's content is licensed under ");
                a[href: "https://creativecommons.org/licenses/by-sa/4.0/"]  {
                    text("CC-BY-SA");
                }
            }
        }
    }
}

fn label(label: &str) -> Node {
    html! {
        span[class: "post-label"] {
            text(label);
        }
    }
}
