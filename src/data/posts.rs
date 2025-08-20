// src/data/posts.rs
#[derive(Clone, Debug, PartialEq)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub excerpt: String,
    pub date: String,
    pub category: String,
    pub tags: Vec<String>,
    pub reading_time: u32,
    pub featured: bool,
}

pub fn get_blog_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            id: 1,
            title: "Getting Started with Dioxus".to_string(),
            content: "<h1>Getting Started with Dioxus</h1><p>Dioxus is an amazing framework for building user interfaces in Rust. It brings the component-based architecture we know from React to the Rust ecosystem.</p><h2>Why Dioxus?</h2><ul><li><strong>Performance</strong>: Compiled to native code</li><li><strong>Safety</strong>: Rust's memory safety guarantees</li><li><strong>Familiar</strong>: React-like syntax and patterns</li><li><strong>Versatile</strong>: Web, desktop, mobile, and TUI support</li></ul><h2>Setting Up</h2><p>Getting started is straightforward:</p><pre><code>cargo new my-dioxus-app\ncd my-dioxus-app\ncargo add dioxus</code></pre><p>Then you can start building components with the familiar JSX-like syntax!</p>".to_string(),
            excerpt: "Learn how to get started with Dioxus, the React-like framework for Rust.".to_string(),
            date: "2025-08-20".to_string(),
            category: "dioxus".to_string(),
            tags: vec!["rust".to_string(), "tutorial".to_string(), "beginner".to_string()],
            reading_time: 5,
            featured: true,
        },
        BlogPost {
            id: 2,
            title: "Building Modular Web Apps in Rust".to_string(),
            content: "<h1>Building Modular Web Apps in Rust</h1><p>One of the key principles of good software architecture is modularity. In this post, I'll share how to structure your Rust web applications for maximum maintainability and extensibility.</p><h2>The Modular Approach</h2><p>When building web applications, we want:</p><ul><li>Easy to add new features</li><li>Clean separation of concerns</li><li>Reusable components</li><li>Simple configuration</li></ul><h2>Theme System Example</h2><p>Here's how I implemented a modular theme system for this blog...</p>".to_string(),
            excerpt: "Explore patterns for building modular and maintainable web applications with Rust.".to_string(),
            date: "2025-08-19".to_string(),
            category: "rust".to_string(),
            tags: vec!["rust".to_string(), "architecture".to_string(), "web-dev".to_string()],
            reading_time: 8,
            featured: false,
        },
        BlogPost {
            id: 3,
            title: "My Journey Learning Rust".to_string(),
            content: "<h1>My Journey Learning Rust</h1><p>Learning Rust has been one of the most rewarding experiences in my programming career. The language's focus on safety and performance, combined with its growing ecosystem, makes it perfect for modern software development.</p><h2>The Beginning</h2><p>I started learning Rust because I was curious about systems programming and wanted to understand how to write safe, fast code...</p><h2>Key Insights</h2><ul><li>Ownership system takes time to understand but is incredibly powerful</li><li>The compiler is your friend, not your enemy</li><li>The community is welcoming and helpful</li><li>Performance benefits are real and measurable</li></ul>".to_string(),
            excerpt: "Personal reflections on learning Rust and building with this amazing language.".to_string(),
            date: "2025-08-18".to_string(),
            category: "personal".to_string(),
            tags: vec!["rust".to_string(), "learning".to_string(), "personal".to_string()],
            reading_time: 6,
            featured: false,
        },
    ]
}