use {
    dioxus::prelude::*,
    crate::{
        routes::Route,
        components::{
            header::Header,
            syntax::HighlighterConfig,
        },
        data::{
            categories::{BlogCategory, get_categories},
            posts::{BlogPost, get_blog_posts, ContentBlock},
        },
    },
};

#[component]
pub fn Post(id: i32) -> Element {
    let posts = get_blog_posts();
    let post = posts.into_iter().find(|p| p.id == id);
    let categories = get_categories();
    let highlighter = HighlighterConfig::new();

    match post {
        Some(post) => {
            let category = categories.iter().find(|c| c.slug == post.category).cloned();

            rsx! {
                div { class: "page",
                    Header {}
                    main { class: "main",
                        div { class: "container",
                            article { class: "post",
                                header {
                                    h1 { class: "post-title", "{post.title}" }
                                    div { class: "post-meta",
                                        time { class: "post-date", "{post.date}" }
                                        if let Some(cat) = category {
                                            Link {
                                                to: Route::CategoryPage { category: cat.slug.clone() },
                                                class: "post-category",
                                                style: "background-color: {cat.color}; color: white;",
                                                "{cat.name}"
                                            }
                                        }
                                        span { class: "post-reading-time", "{post.reading_time} min read" }
                                    }
                                }
                                div { class: "post-content",
                                    for block in post.content.iter() {
                                        match block {
                                            ContentBlock::Prose(html) => rsx! {
                                                div { dangerous_inner_html: "{html}" }
                                            },
                                            ContentBlock::Code { code, language } => {
                                                let highlighted_code = highlighter.highlight_code(code, language.as_deref());
                                                rsx! {
                                                    pre { class: "code-block",
                                                        code { dangerous_inner_html: "{highlighted_code}" }
                                                    }
                                                }
                                            },
                                        }
                                    }
                                }
                                if !post.tags.is_empty() {
                                    div { class: "post-tags",
                                        for tag in post.tags {
                                            span { class: "tag", "{tag}" }
                                        }
                                    }
                                }
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
                            h1 { "Post Not Found" }
                            p { "Sorry, the post you're looking for doesn't exist." }
                        }
                    }
                }
            }
        },
    }
}