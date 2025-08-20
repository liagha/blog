use {
    crate::{
        Map, Regex,
    },
};

pub struct HighlighterConfig {
    languages: Map<String, LanguageConfig>,
}

pub struct LanguageConfig {
    keywords: Vec<String>,
    comment_pattern: Regex,
    string_pattern: Regex,
    number_pattern: Regex,
}

impl HighlighterConfig {
    pub fn new() -> Self {
        let mut languages = Map::new();

        languages.insert(
            "rust".to_string(),
            LanguageConfig {
                keywords: vec![
                    "fn", "let", "mut", "pub", "struct", "enum", "impl", "use", "mod",
                    "if", "else", "for", "while", "loop", "return", "match", "self"
                ].into_iter().map(String::from).collect(),
                comment_pattern: Regex::new(r"//.*$|/\*[\s\S]*?\*/").unwrap(),
                string_pattern: Regex::new(r#""[^"]*""#).unwrap(),
                number_pattern: Regex::new(r"\b\d+\.?\d*\b").unwrap(),
            },
        );

        languages.insert(
            "bash".to_string(),
            LanguageConfig {
                keywords: vec![
                    "if", "then", "else", "fi", "for", "while", "do", "done", "echo",
                    "export", "source"
                ].into_iter().map(String::from).collect(),
                comment_pattern: Regex::new(r"#.*$").unwrap(),
                string_pattern: Regex::new(r#""[^"]*"|'[^']*'"#).unwrap(),
                number_pattern: Regex::new(r"\b\d+\b").unwrap(),
            },
        );

        HighlighterConfig { languages }
    }

    pub fn highlight_code(&self, code: &str, language: Option<&str>) -> String {
        let lang = language.unwrap_or("plaintext");
        let mut highlighted = code.to_string();

        if let Some(config) = self.languages.get(lang) {
            highlighted = config.comment_pattern.replace_all(&highlighted, "<span class=\"comment\">$0</span>").to_string();
            highlighted = config.string_pattern.replace_all(&highlighted, "<span class=\"string\">$0</span>").to_string();
            highlighted = config.number_pattern.replace_all(&highlighted, "<span class=\"number\">$0</span>").to_string();
            for keyword in &config.keywords {
                let pattern = Regex::new(&format!(r"\b{}\b", regex::escape(keyword))).unwrap();
                highlighted = pattern.replace_all(&highlighted, format!("<span class=\"keyword\">{}</span>", keyword)).to_string();
            }
        }

        highlighted
    }
}