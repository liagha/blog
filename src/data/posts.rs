pub use {
    crate::{
        markdown::{CodeBlockKind, Event, html, Parser, Tag, TagEnd},
    }
};

#[derive(Clone, Debug, PartialEq)]
pub enum ContentBlock {
    Prose(String),
    Code { code: String, language: Option<String> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: Vec<ContentBlock>,
    pub excerpt: String,
    pub date: String,
    pub category: String,
    pub tags: Vec<String>,
    pub reading_time: u32,
    pub featured: bool,
}

fn parse_markdown_to_blocks(md: &str) -> Vec<ContentBlock> {
    let parser = Parser::new(md);
    let mut blocks: Vec<ContentBlock> = vec![];
    let mut current_events: Vec<Event> = vec![];
    let mut current_code = String::new();
    let mut current_lang: Option<String> = None;
    let mut in_code_block = false;

    for event in parser {
        if in_code_block {
            match event {
                Event::Text(text) => {
                    current_code.push_str(&text);
                }
                Event::End(TagEnd::CodeBlock) => {
                    blocks.push(ContentBlock::Code {
                        code: std::mem::take(&mut current_code),
                        language: current_lang.take(),
                    });
                    in_code_block = false;
                }
                _ => {}
            }
        } else {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    if !current_events.is_empty() {
                        let mut html_buf = String::new();
                        html::push_html(&mut html_buf, current_events.into_iter());
                        blocks.push(ContentBlock::Prose(html_buf));
                        current_events = vec![];
                    }
                    in_code_block = true;
                    current_code = String::new();
                    current_lang = match kind {
                        CodeBlockKind::Fenced(info) if !info.is_empty() => Some(info.split_whitespace().next().unwrap().to_string()),
                        _ => None,
                    };
                }
                other => {
                    current_events.push(other);
                }
            }
        }
    }

    if !current_events.is_empty() {
        let mut html_buf = String::new();
        html::push_html(&mut html_buf, current_events.into_iter());
        blocks.push(ContentBlock::Prose(html_buf));
    }

    blocks
}

pub fn get_blog_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            id: 2,
            title: "Building Modular Web Apps in Rust".to_string(),
            content: vec![
                ContentBlock::Prose("<h1>Building Modular Web Apps in Rust</h1><p>One of the key principles of good software architecture is modularity. In this post, I'll share how to structure your Rust web applications for maximum maintainability and extensibility.</p><h2>The Modular Approach</h2><p>When building web applications, we want:</p><ul><li>Easy to add new features</li><li>Clean separation of concerns</li><li>Reusable components</li><li>Simple configuration</li></ul><h2>Theme System Example</h2><p>Here's how I implemented a modular theme system for this blog...</p>".to_string()),
            ],
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
            content: vec![
                ContentBlock::Prose("<h1>My Journey Learning Rust</h1><p>Learning Rust has been one of the most rewarding experiences in my programming career. The language's focus on safety and performance, combined with its growing ecosystem, makes it perfect for modern software development.</p><h2>The Beginning</h2><p>I started learning Rust because I was curious about systems programming and wanted to understand how to write safe, fast code...</p><h2>Key Insights</h2><ul><li>Ownership system takes time to understand but is incredibly powerful</li><li>The compiler is your friend, not your enemy</li><li>The community is welcoming and helpful</li><li>Performance benefits are real and measurable</li></ul>".to_string()),
            ],
            excerpt: "Personal reflections on learning Rust and building with this amazing language.".to_string(),
            date: "2025-08-18".to_string(),
            category: "personal".to_string(),
            tags: vec!["rust".to_string(), "learning".to_string(), "personal".to_string()],
            reading_time: 6,
            featured: false,
        },
        BlogPost {
            id: 4,
            title: "Exploring Markdown in Dioxus".to_string(),
            content: parse_markdown_to_blocks(r#"
# Exploring Markdown in Dioxus

This is a sample post written entirely in Markdown. It supports headings, lists, and code blocks just like your HTML posts.

## Why Add Markdown Support?

- Easier content creation
- Better for writers without HTML knowledge
- Maintains the same rendering pipeline

Here's a Rust code example:

```rust
fn main() {
    println!("Hello from Markdown!");
}
```

And a bash script:

```bash
echo "Markdown support added!"
```

That's it! More content can follow.
"#),
            excerpt: "A demonstration of Markdown post support in your Dioxus blog.".to_string(),
            date: "2025-08-21".to_string(),
            category: "".to_string(),
            tags: vec!["markdown".to_string(), "dioxus".to_string(), "rust".to_string()],
            reading_time: 4,
            featured: true,
        },
    ]
}