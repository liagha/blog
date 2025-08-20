#[derive(Clone, Debug, PartialEq)]
pub struct BlogCategory {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub color: String,
}

pub fn get_categories() -> Vec<BlogCategory> {
    vec![
        BlogCategory {
            name: "Rust".to_string(),
            slug: "rust".to_string(),
            description: "Articles about Rust programming language".to_string(),
            color: "#ce422b".to_string(),
        },
        BlogCategory {
            name: "Web Development".to_string(),
            slug: "web-dev".to_string(),
            description: "Web development tutorials and insights".to_string(),
            color: "#61dafb".to_string(),
        },
        BlogCategory {
            name: "Personal".to_string(),
            slug: "personal".to_string(),
            description: "Personal thoughts and experiences".to_string(),
            color: "#8b5cf6".to_string(),
        },
    ]
}