#[derive(Clone, Debug, PartialEq)]
pub struct StaticPage {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub show_in_nav: bool,
}

pub fn get_static_pages() -> Vec<StaticPage> {
    vec![
        StaticPage {
            title: "About".to_string(),
            slug: "about".to_string(),
            content: "<h1>About Me</h1><p>I'm a developer passionate about Rust and web technologies. This blog is built with Dioxus, showcasing the power of Rust for web development.</p><h2>Skills</h2><ul><li>Rust programming</li><li>Web development with Dioxus</li><li>System programming</li><li>Frontend technologies</li></ul><h2>Contact</h2><p>Feel free to reach out if you want to discuss Rust, web development, or any interesting projects!</p>".to_string(),
            show_in_nav: true,
        },
        StaticPage {
            title: "Projects".to_string(),
            slug: "projects".to_string(),
            content: "<h1>My Projects</h1><p>Here are some projects I've been working on:</p><h2>This Blog</h2><p>Built with Dioxus and Rust, featuring:</p><ul><li>Modular theme system</li><li>Category organization</li><li>Responsive design</li></ul><h2>Other Projects</h2><p>More projects coming soon...</p>".to_string(),
            show_in_nav: true,
        },
        StaticPage {
            title: "Privacy".to_string(),
            slug: "privacy".to_string(),
            content: "<h1>Privacy Policy</h1><p>This is a simple blog that doesn't collect personal data. No cookies, no tracking, just content.</p>".to_string(),
            show_in_nav: false,
        },
    ]
}