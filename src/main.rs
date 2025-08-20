mod app;
mod routes;
mod components;
mod pages;
mod data;

pub mod markdown {
    pub use pulldown_cmark::{CodeBlockKind, Event, html, Parser, Tag, TagEnd};
}

pub type Regex = regex::Regex;
pub type Map<Key, Value> = std::collections::HashMap<Key, Value>;

fn main() {
    use dioxus::prelude::*;
    use app::App;

    launch(App);
}