use {
    dioxus::{
        prelude::*,
        document::Stylesheet,
    },
    crate::{
        Map,
        routes::Route,
        data::themes::get_themes,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub name: String,
    pub primary_bg: String,
    pub secondary_bg: String,
    pub primary_text: String,
    pub secondary_text: String,
    pub accent: String,
    pub border: String,
    pub shadow: String,
    pub hover_shadow: String,
}

impl Default for Theme {
    fn default() -> Theme {
        get_themes().get("dark").unwrap().clone()
    }
}

impl Theme {
    pub fn to_css_variables(&self) -> String {
        format!(
            ":root {{
                --primary-bg: {};
                --secondary-bg: {};
                --primary-text: {};
                --secondary-text: {};
                --accent: {};
                --border: {};
                --shadow: {};
                --hover-shadow: {};
            }}",
            self.primary_bg,
            self.secondary_bg,
            self.primary_text,
            self.secondary_text,
            self.accent,
            self.border,
            self.shadow,
            self.hover_shadow
        )
    }
}

#[derive(Clone)]
pub struct AppState {
    pub current_theme: String,
    pub available_themes: Map<String, Theme>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_theme: "light".to_string(),
            available_themes: get_themes(),
        }
    }
}

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::new()));

    let app_state = use_context::<Signal<AppState>>();
    let current_theme = app_state.read().available_themes.get(&app_state.read().current_theme).cloned().unwrap_or_default();

    rsx! {
        Stylesheet { href: asset!("./public/styles.css") }
        style { "{current_theme.to_css_variables()}" }
        Router::<Route> {}
    }
}