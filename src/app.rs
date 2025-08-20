use {
    dioxus::prelude::*,
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

    const BASE_CSS: &str = "
        * { box-sizing: border-box; }

        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 0;
            background-color: var(--primary-bg);
            color: var(--primary-text);
            transition: background-color 0.3s ease, color 0.3s ease;
        }

        .page {
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }

        .container {
            max-width: 800px;
            margin: 0 auto;
            padding: 0 20px;
            width: 100%;
        }

        .main { flex: 1; padding: 2rem 0; }

        .header {
            background-color: var(--secondary-bg);
            border-bottom: 1px solid var(--border);
            padding: 1rem 0;
            margin-bottom: 0;
        }

        .header-content {
            display: flex;
            justify-content: space-between;
            align-items: center;
            flex-wrap: wrap;
            gap: 1rem;
        }

        .site-title {
            color: var(--primary-text);
            margin: 0;
            font-size: 2rem;
            font-weight: 700;
        }

        .nav {
            display: flex;
            gap: 1.5rem;
            align-items: center;
            flex-wrap: wrap;
        }

        .nav-link {
            color: var(--accent);
            text-decoration: none;
            font-weight: 500;
            padding: 0.5rem 0;
            transition: color 0.2s ease;
        }

        .nav-link:hover { color: var(--primary-text); }

        .theme-selector {
            display: flex;
            gap: 0.5rem;
            align-items: center;
        }

        .theme-btn {
            background: var(--accent);
            color: var(--secondary-bg);
            border: none;
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
            cursor: pointer;
            font-size: 0.75rem;
            transition: opacity 0.2s ease;
        }

        .theme-btn:hover { opacity: 0.8; }
        .theme-btn.active { opacity: 1; font-weight: bold; }

        .posts-grid {
            display: flex;
            flex-direction: column;
            gap: 1.5rem;
        }

        .post-preview {
            background: var(--secondary-bg);
            padding: 1.5rem;
            border-radius: 8px;
            box-shadow: 0 2px 4px var(--shadow);
            border: 1px solid var(--border);
            transition: transform 0.2s ease, box-shadow 0.2s ease;
        }

        .post-preview:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px var(--hover-shadow);
        }

        .post-title {
            margin: 0 0 0.5rem 0;
            font-size: 1.5rem;
            font-weight: 600;
            line-height: 1.3;
        }

        .post-link {
            color: var(--primary-text);
            text-decoration: none;
            transition: color 0.2s ease;
        }

        .post-link:hover { color: var(--accent); }

        .post-meta {
            display: flex;
            gap: 1rem;
            align-items: center;
            margin-bottom: 1rem;
            flex-wrap: wrap;
        }

        .post-date {
            color: var(--secondary-text);
            font-size: 0.875rem;
        }

        .post-category {
            display: inline-block;
            padding: 0.25rem 0.5rem;
            border-radius: 12px;
            font-size: 0.75rem;
            font-weight: 600;
            text-decoration: none;
        }

        .post-reading-time {
            color: var(--secondary-text);
            font-size: 0.75rem;
        }

        .post-excerpt {
            color: var(--secondary-text);
            margin: 0;
            line-height: 1.5;
        }

        .post-tags {
            display: flex;
            gap: 0.5rem;
            margin-top: 1rem;
            flex-wrap: wrap;
        }

        .tag {
            background: var(--border);
            color: var(--secondary-text);
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
            font-size: 0.75rem;
        }

        .featured-badge {
            background: var(--accent);
            color: var(--secondary-bg);
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
            font-size: 0.75rem;
            font-weight: bold;
        }

        .post {
            background: var(--secondary-bg);
            padding: 2rem;
            border-radius: 8px;
            box-shadow: 0 2px 4px var(--shadow);
            border: 1px solid var(--border);
        }

        .post .post-title {
            font-size: 2.25rem;
            margin-bottom: 1rem;
            color: var(--primary-text);
        }

        .post-content {
            font-size: 1.1rem;
            line-height: 1.7;
            color: var(--secondary-text);
        }

        .post-content h1, .post-content h2, .post-content h3 {
            color: var(--primary-text);
            margin-top: 2rem;
            margin-bottom: 1rem;
        }

        .post-content p { margin-bottom: 1.5rem; }

        .post-content code {
            background: var(--border);
            padding: 0.2rem 0.4rem;
            border-radius: 4px;
            font-family: 'Courier New', monospace;
        }

        .post-content pre {
            background: var(--border);
            padding: 1rem;
            border-radius: 8px;
            overflow-x: auto;
            margin: 1.5rem 0;
        }

        .code-block {
            background: var(--border);
            padding: 1rem;
            border-radius: 8px;
            overflow-x: auto;
            margin: 1.5rem 0;
            font-family: 'Courier New', monospace;
        }

        .code-block code {
            background: none;
            padding: 0;
            font-size: 0.9rem;
        }

        .keyword { color: #ff6b35; font-weight: bold; } /* Rust/Dioxus orange */
        .comment { color: #6b7280; font-style: italic; } /* Neutral gray */
        .string { color: #10b981; } /* Green for strings */
        .number { color: #3b82f6; } /* Blue for numbers */

        .section-title {
            color: var(--primary-text);
            margin: 0 0 1.5rem 0;
            font-size: 1.5rem;
            font-weight: 600;
        }

        .category-header {
            margin-bottom: 2rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid var(--border);
        }

        .category-description {
            color: var(--secondary-text);
            margin: 0.5rem 0 0 0;
        }

        .error-page {
            text-align: center;
            padding: 3rem 1.5rem;
            background: var(--secondary-bg);
            border-radius: 8px;
            box-shadow: 0 2px 4px var(--shadow);
        }

        @media (max-width: 768px) {
            .header-content { flex-direction: column; align-items: flex-start; }
            .nav { width: 100%; }
            .theme-selector { margin-top: 0.5rem; }
            .container { padding: 0 1rem; }
            .post { padding: 1.5rem; }
            .main { padding: 1rem 0; }
        }
    ";

    rsx! {
        style { "{current_theme.to_css_variables()}" }
        style { "{BASE_CSS}" }
        Router::<Route> {}
    }
}