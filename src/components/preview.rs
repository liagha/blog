use {
    dioxus::prelude::*,
    crate::{
        routes::Route,
        data::{
            categories::{BlogCategory, get_categories},
            posts::{BlogPost, Tag},
        },
    }
};

#[component]
pub fn PostPreview(post: BlogPost) -> Element {
    let categories = get_categories();
    let category = categories.iter().find(|c| c.slug == post.category).cloned();

    rsx! {
        article { class: "post-preview",
            header {
                h2 { class: "post-title",
                    Link {
                        to: Route::Post { id: post.id },
                        class: "post-link",
                        "{post.title}"
                    }
                }
                div { class: "post-meta",
                    time { class: "post-date", "{post.date}" }
                    if let Some(cat) = category {
                        Link {
                            to: Route::CategoryPage { category: cat.slug.clone() },
                            class: "post-category",
                            style: format!("background-color: {}; color: white;", cat.color.to_css()),
                            "{cat.name}"
                        }
                    }
                    span { class: "post-reading-time", "{post.reading_time} min read" }
                    if post.featured {
                        span { class: "featured-badge", "Featured" }
                    }
                }
            }
            p { class: "post-excerpt", "{post.excerpt}" }
            if !post.tags.is_empty() {
                div { class: "post-tags",
                    for tag in post.tags {
                        span {
                            class: "tag",
                            style: format!("background-color: {}; color: white;", tag.color.to_css()),
                            "{tag.name}"
                        }
                    }
                }
            }
        }
    }
}