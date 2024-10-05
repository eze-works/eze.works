use crate::post::{Post, PostStage};
use rand::seq::SliceRandom;
use rand::thread_rng;
use toph::{html, text, Node};

pub fn home(posts: &[Post]) -> Node {
    let published = posts
        .iter()
        .filter(|p| p.metadata.stage == PostStage::Published);
    super::base_layout(
        html! {
            div[class: "featured-quote"] {
                featured_quote();
            }

            div [class: "post-list center"] {
                published.map(post_card);
            }
        },
        super::BaseLayoutOptions::default(),
    )
}

fn post_card(post: &Post) -> Node {
    html! {
        div[class: "post-card"] {
            span[class: "post-card-date"] {
                text(post.metadata.date.strftime("%b %d, %Y"));
                text(" »");
            }
            a[class: "post-card-title", href: format!("/post/{}", post.slug)] {
                text(&post.metadata.title);
            }
        }
    }
}

struct Quote {
    text: &'static str,
    from: &'static str,
}

const QUOTES: [Quote; 2] = [
    Quote {
        text: "We build our computers the way we build our cities – over time, without a plan, on top of ruins.",
        from: "Ellen Ullman, Life in Code"
    },
    Quote {
        text: "Give me six hours to chop down a tree and I will spend the first four sharpening the axe.",
        from: "Unclear ¯\\_(ツ)_/¯"
    }
];

fn featured_quote() -> Node {
    let mut rng = thread_rng();
    let choice = QUOTES.choose(&mut rng);
    let quote = choice.map(|c| html! { p { text(c.text); } });
    let from = choice.map(|c| html! { p { text("- "); text(c.from); } });
    html! {
        quote;
        from;
    }
}
