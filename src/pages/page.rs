use dioxus::prelude::*;
use crate::components::header::Header;
use crate::data::pages::{StaticPage, get_static_pages};

#[component]
pub fn Page(slug: String) -> Element {
    let static_pages = get_static_pages();
    let page = static_pages.into_iter().find(|p| p.slug == slug);

    match page {
        Some(page) => rsx! {
            div { class: "page",
                Header {}
                main { class: "main",
                    div { class: "container",
                        article { class: "post",
                            header {
                                h1 { class: "post-title", "{page.title}" }
                            }
                            div {
                                class: "post-content",
                                dangerous_inner_html: "{page.content}"
                            }
                        }
                    }
                }
            }
        },
        None => rsx! {
            div { class: "page",
                Header {}
                main { class: "main",
                    div { class: "container",
                        div { class: "error-page",
                            h1 { "Page Not Found" }
                            p { "Sorry, the page you're looking for doesn't exist." }
                        }
                    }
                }
            }
        },
    }
}