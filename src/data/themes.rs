use {
    crate::{
        Map, app::Theme,
    },
};

pub fn get_themes() -> Map<String, Theme> {
    let mut themes = Map::new();

    themes.insert("light".to_string(), Theme {
        name: "Light".to_string(),
        primary_bg: "rgba(248, 249, 250, 1)".to_string(),
        secondary_bg: "rgba(255, 255, 255, 1)".to_string(),
        primary_text: "rgba(33, 37, 41, 1)".to_string(),
        secondary_text: "rgba(108, 117, 125, 1)".to_string(),
        accent: "rgba(0, 123, 255, 1)".to_string(),
        border: "rgba(233, 236, 239, 1)".to_string(),
        shadow: "rgba(0, 0, 0, 0.1)".to_string(),
        hover_shadow: "rgba(0, 0, 0, 0.15)".to_string(),
    });

    themes.insert("dark".to_string(), Theme {
        name: "Dark".to_string(),
        primary_bg: "rgba(26, 29, 35, 1)".to_string(),
        secondary_bg: "rgba(45, 55, 72, 1)".to_string(),
        primary_text: "rgba(247, 250, 252, 1)".to_string(),
        secondary_text: "rgba(160, 174, 192, 1)".to_string(),
        accent: "rgba(66, 153, 225, 1)".to_string(),
        border: "rgba(74, 85, 104, 1)".to_string(),
        shadow: "rgba(0, 0, 0, 0.3)".to_string(),
        hover_shadow: "rgba(0, 0, 0, 0.4)".to_string(),
    });

    themes.insert("ocean".to_string(), Theme {
        name: "Ocean".to_string(),
        primary_bg: "rgba(15, 23, 42, 1)".to_string(),
        secondary_bg: "rgba(30, 41, 59, 1)".to_string(),
        primary_text: "rgba(241, 245, 249, 1)".to_string(),
        secondary_text: "rgba(148, 163, 184, 1)".to_string(),
        accent: "rgba(14, 165, 233, 1)".to_string(),
        border: "rgba(51, 65, 85, 1)".to_string(),
        shadow: "rgba(14, 165, 233, 0.1)".to_string(),
        hover_shadow: "rgba(14, 165, 233, 0.2)".to_string(),
    });

    themes.insert("forest".to_string(), Theme {
        name: "Forest".to_string(),
        primary_bg: "rgba(15, 26, 15, 1)".to_string(),
        secondary_bg: "rgba(26, 46, 26, 1)".to_string(),
        primary_text: "rgba(240, 249, 240, 1)".to_string(),
        secondary_text: "rgba(163, 201, 163, 1)".to_string(),
        accent: "rgba(16, 185, 129, 1)".to_string(),
        border: "rgba(45, 90, 45, 1)".to_string(),
        shadow: "rgba(16, 185, 129, 0.1)".to_string(),
        hover_shadow: "rgba(16, 185, 129, 0.2)".to_string(),
    });

    themes
}