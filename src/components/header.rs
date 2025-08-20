use {
    dioxus::prelude::*,
    
    crate::{
        Map,
        routes::Route,
        app::{AppState, Theme},
        data::pages::get_static_pages,
    },
};

#[component]
pub fn Header() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let static_pages = get_static_pages();
    let nav_pages: Vec<_> = static_pages.into_iter().filter(|p| p.show_in_nav).collect();
    let themes: Map<String, Theme> = app_state.read().available_themes.clone();
    let current_theme = app_state.read().current_theme.clone();

    rsx! {
        header { class: "header",
            div { class: "container",
                div { class: "header-content",
                    div {
                        h1 { class: "site-title", "My Smart Blog" }
                    }
                    nav { class: "nav",
                        Link { to: Route::Home {}, class: "nav-link", "Home" }
                        for page in nav_pages {
                            Link {
                                to: Route::Page { slug: page.slug.clone() },
                                class: "nav-link",
                                "{page.title}"
                            }
                        }
                        div { class: "theme-selector",
                            for (theme_key, theme) in themes {
                                button {
                                    class: if theme_key == current_theme { "theme-btn active" } else { "theme-btn" },
                                    onclick: move |_| {
                                        app_state.write().current_theme = theme_key.clone();
                                    },
                                    "{theme.name}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}