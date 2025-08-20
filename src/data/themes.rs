use {
    crate::{
        Map, app::Theme,  
    },
};

pub fn get_themes() -> Map<String, Theme> {
    let mut themes = Map::new();

    themes.insert("light".to_string(), Theme {
        name: "Light".to_string(),
        primary_bg: "#f8f9fa".to_string(),
        secondary_bg: "#ffffff".to_string(),
        primary_text: "#212529".to_string(),
        secondary_text: "#6c757d".to_string(),
        accent: "#007bff".to_string(),
        border: "#e9ecef".to_string(),
        shadow: "rgba(0, 0, 0, 0.1)".to_string(),
        hover_shadow: "rgba(0, 0, 0, 0.15)".to_string(),
    });

    themes.insert("dark".to_string(), Theme {
        name: "Dark".to_string(),
        primary_bg: "#1a1d23".to_string(),
        secondary_bg: "#2d3748".to_string(),
        primary_text: "#f7fafc".to_string(),
        secondary_text: "#a0aec0".to_string(),
        accent: "#4299e1".to_string(),
        border: "#4a5568".to_string(),
        shadow: "rgba(0, 0, 0, 0.3)".to_string(),
        hover_shadow: "rgba(0, 0, 0, 0.4)".to_string(),
    });

    themes.insert("ocean".to_string(), Theme {
        name: "Ocean".to_string(),
        primary_bg: "#0f172a".to_string(),
        secondary_bg: "#1e293b".to_string(),
        primary_text: "#f1f5f9".to_string(),
        secondary_text: "#94a3b8".to_string(),
        accent: "#0ea5e9".to_string(),
        border: "#334155".to_string(),
        shadow: "rgba(14, 165, 233, 0.1)".to_string(),
        hover_shadow: "rgba(14, 165, 233, 0.2)".to_string(),
    });

    themes.insert("forest".to_string(), Theme {
        name: "Forest".to_string(),
        primary_bg: "#0f1a0f".to_string(),
        secondary_bg: "#1a2e1a".to_string(),
        primary_text: "#f0f9f0".to_string(),
        secondary_text: "#a3c9a3".to_string(),
        accent: "#10b981".to_string(),
        border: "#2d5a2d".to_string(),
        shadow: "rgba(16, 185, 129, 0.1)".to_string(),
        hover_shadow: "rgba(16, 185, 129, 0.2)".to_string(),
    });

    themes
}