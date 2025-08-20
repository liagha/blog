use {
    dioxus::prelude::*,
    crate::{
        components::{
            header::Header,
            post_preview::PostPreview,
        },
        data::{
            categories::{BlogCategory, get_categories},
            posts::{BlogPost, get_blog_posts},
        },
    }    
};

#[component]
pub fn CategoryPage(category: String) -> Element {
    let posts: Vec<BlogPost> = get_blog_posts();
    let categories = get_categories();
    let category_info = categories.iter().find(|c| c.slug == category).cloned();
    let category_posts: Vec<BlogPost> = posts.into_iter().filter(|p| p.category == category).collect();

    match category_info {
        Some(cat_info) => rsx! {
            div { class: "page",
                Header {}
                main { class: "main",
                    div { class: "container",
                        div { class: "category-header",
                            h1 { class: "section-title",
                                style: "color: {cat_info.color};",
                                "{cat_info.name}"
                            }
                            p { class: "category-description", "{cat_info.description}" }
                        }
                        section { class: "posts-section",
                            div { class: "posts-grid",
                                for post in category_posts {
                                    PostPreview { post }
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
                            h1 { "Category Not Found" }
                            p { "Sorry, the category you're looking for doesn't exist." }
                        }
                    }
                }
            }
        },
    }
}