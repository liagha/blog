use {
    dioxus::prelude::*,
    crate::{
        components::{
            header::Header,
            preview::PostPreview,
        },
        data::posts::{BlogPost, get_blog_posts},
    }
};

#[component]
pub fn Home() -> Element {
    let posts: Vec<BlogPost> = get_blog_posts();
    let featured_posts: Vec<BlogPost> = posts.iter().filter(|p| p.featured).cloned().collect();
    let recent_posts: Vec<BlogPost> = posts.into_iter().take(5).collect();

    rsx! {
        div { class: "page",
            Header {}
            main { class: "main",
                div { class: "container",
                    if !featured_posts.is_empty() {
                        section { class: "posts-section",
                            h2 { class: "section-title", "Featured Posts" }
                            div { class: "posts-grid",
                                for post in featured_posts {
                                    PostPreview { post }
                                }
                            }
                        }
                    }
                    section { class: "posts-section",
                        h2 { class: "section-title", "Latest Posts" }
                        div { class: "posts-grid",
                            for post in recent_posts {
                                PostPreview { post }
                            }
                        }
                    }
                }
            }
        }
    }
}