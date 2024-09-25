use toph::{html, text, Node};

pub fn not_found() -> Node {
    super::base_layout(html! {
        div[class: "center not-found"] {
            h1 {
                text("The page you requested could not be found");
            }
        }
    }, super::BaseLayoutOptions::default())
}
