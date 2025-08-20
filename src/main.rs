use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/post/:id")]
    Post { id: i32 },
}

#[component]
fn App() -> Element {
    const CSS: &str = include_str!("../public/styles.css");

    rsx! {
        style { "{CSS}" }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let posts = vec![
        (1, "First Post", "This is my first blog post!", "2025-08-20"),
        (2, "Second Post", "Another post about Rust and Dioxus.", "2025-08-19"),
    ];

    rsx! {
        div { class: "container",
            h1 { "My Simple Blog" }
            nav {
                Link { to: Route::Home {}, "Home" }
            }
            div { class: "posts",
                for (id, title, _, date) in posts {
                    div { class: "post-preview",
                        h2 {
                            Link { to: Route::Post { id }, "{title}" }
                        }
                        p { class: "date", "{date}" }
                    }
                }
            }
        }
    }
}

#[component]
fn Post(id: i32) -> Element {
    let posts = vec![
        (1, "First Post", "This is my first blog post! Welcome to my blog built with Dioxus. It's simple, fast, and written in Rust.", "2025-08-20"),
        (2, "Second Post", "Another post about Rust and Dioxus. I love how easy it is to build web apps with this framework!", "2025-08-19"),
    ];

    let post = posts.into_iter().find(|(post_id, _, _, _)| *post_id == id);

    match post {
        Some((_, title, content, date)) => rsx! {
            div { class: "container",
                h1 { "{title}" }
                nav {
                    Link { to: Route::Home {}, "Back to Home" }
                }
                p { class: "date", "{date}" }
                p { "{content}" }
            }
        },
        None => rsx! {
            div { class: "container",
                h1 { "Post Not Found" }
                nav {
                    Link { to: Route::Home {}, "Back to Home" }
                }
            }
        },
    }
}