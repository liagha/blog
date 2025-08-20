use regex::Regex;
use std::collections::HashMap;

pub struct HighlighterConfig {
    languages: HashMap<String, LanguageConfig>,
}

pub struct LanguageConfig {
    keywords: Vec<String>,
    comment_pattern: Regex,
    string_pattern: Regex,
    number_pattern: Regex,
}

impl HighlighterConfig {
    pub fn new() -> Self {
        let mut languages = HashMap::new();

        // Rust language configuration
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

        // Bash language configuration
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
            // Highlight comments
            highlighted = config.comment_pattern.replace_all(&highlighted, "<span class=\"comment\">$0</span>").to_string();
            // Highlight strings
            highlighted = config.string_pattern.replace_all(&highlighted, "<span class=\"string\">$0</span>").to_string();
            // Highlight numbers
            highlighted = config.number_pattern.replace_all(&highlighted, "<span class=\"number\">$0</span>").to_string();
            // Highlight keywords
            for keyword in &config.keywords {
                let pattern = Regex::new(&format!(r"\b{}\b", regex::escape(keyword))).unwrap();
                highlighted = pattern.replace_all(&highlighted, format!("<span class=\"keyword\">{}</span>", keyword)).to_string();
            }
        }

        highlighted
    }
}