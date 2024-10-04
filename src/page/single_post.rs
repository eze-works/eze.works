use crate::post::Post;
use toph::{html, raw_text, text, Node};

pub fn single_post(post: &Post) -> Node {
    super::base_layout(
        html! {
            div[class: "post-container center"] {
                div[class: "post-meta"] {
                    h1[class: "post-title"] {
                        text(&post.metadata.title);
                    }
                    span[class: "post-date"] {
                        text(post.metadata.date.strftime("%b %d, %Y"));

                    }
                }
                div[class: "post-content"] {
                    raw_text(&post.content);
                }
            }
        },
        super::BaseLayoutOptions {
            title: post.metadata.title.clone(),
        },
    )
}
