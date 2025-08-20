use {
    crate::data::posts::Rgba,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BlogCategory {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub color: Rgba,
}

pub fn get_categories() -> Vec<BlogCategory> {
    vec![
        BlogCategory {
            name: String::from("Rust"),
            slug: String::from("rust"),
            description: String::from("Articles about Rust programming language and its ecosystem"),
            color: Rgba { r: 222, g: 82, b: 70, a: 1.0 },
        },
        BlogCategory {
            name: String::from("Personal"),
            slug: String::from("personal"),
            description: String::from("Personal experiences and stories"),
            color: Rgba { r: 107, g: 114, b: 128, a: 1.0 },
        },
    ]
}