use {
    crate::{
        markdown::{self, CodeBlockKind, Event, html, Parser, TagEnd},
    }
};

#[derive(Clone, Debug, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Rgba {
    pub fn to_css(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub color: Rgba,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContentBlock {
    Prose(String),
    Code { code: String, language: Option<String> },
    Image { src: String, alt: String, caption: Option<String> },
    Video { src: String, poster: Option<String>, controls: bool },
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: Vec<ContentBlock>,
    pub excerpt: String,
    pub date: String,
    pub category: String,
    pub tags: Vec<Tag>,
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
                Event::Start(markdown::Tag::CodeBlock(kind)) => {
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
                Event::Start(markdown::Tag::Image { dest_url, title, .. }) => {
                    if !current_events.is_empty() {
                        let mut html_buf = String::new();
                        html::push_html(&mut html_buf, current_events.into_iter());
                        blocks.push(ContentBlock::Prose(html_buf));
                        current_events = vec![];
                    }
                    blocks.push(ContentBlock::Image {
                        src: dest_url.to_string(),
                        alt: title.to_string(),
                        caption: None,
                    });
                }
                Event::Start(markdown::Tag::HtmlBlock) => {
                    if !current_events.is_empty() {
                        let mut html_buf = String::new();
                        html::push_html(&mut html_buf, current_events.into_iter());
                        blocks.push(ContentBlock::Prose(html_buf));
                        current_events = vec![];
                    }
                    current_events.push(event);
                }
                Event::Text(text) if text.starts_with("<video") => {
                    let src = text
                        .split("src=\"")
                        .nth(1)
                        .and_then(|s| s.split('"').next())
                        .map(String::from)
                        .unwrap_or_default();
                    let poster = text
                        .split("poster=\"")
                        .nth(1)
                        .and_then(|s| s.split('"').next())
                        .map(String::from);
                    let controls = text.contains("controls");
                    blocks.push(ContentBlock::Video { src, poster, controls });
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
            title: String::from("Building Modular Web Apps in Rust"),
            content: vec![
                ContentBlock::Prose(String::from("<h1>Building Modular Web Apps in Rust</h1><p>One of the key principles of good software architecture is modularity. In this post, I'll share how to structure your Rust web applications for maximum maintainability and extensibility.</p><h2>The Modular Approach</h2><p>When building web applications, we want:</p><ul><li>Easy to add new features</li><li>Clean separation of concerns</li><li>Reusable components</li><li>Simple configuration</li></ul><h2>Theme System Example</h2><p>Here's how I implemented a modular theme system for this blog...</p>")),
                ContentBlock::Image {
                    src: String::from("/images/rust-modular.jpg"),
                    alt: String::from("Rust modular architecture diagram"),
                    caption: Some(String::from("Diagram showing modular Rust architecture")),
                },
            ],
            excerpt: String::from("Explore patterns for building modular and maintainable web applications with Rust."),
            date: String::from("2025-08-19"),
            category: String::from("rust"),
            tags: vec![
                Tag { name: String::from("rust"), color: Rgba { r: 222, g: 82, b: 70, a: 1.0 } },
                Tag { name: String::from("architecture"), color: Rgba { r: 59, g: 130, b: 246, a: 1.0 } },
                Tag { name: String::from("web-dev"), color: Rgba { r: 16, g: 185, b: 129, a: 1.0 } },
            ],
            reading_time: 8,
            featured: false,
        },
        BlogPost {
            id: 3,
            title: String::from("My Journey Learning Rust"),
            content: vec![
                ContentBlock::Prose(String::from("<h1>My Journey Learning Rust</h1><p>Learning Rust has been one of the most rewarding experiences in my programming career. The language's focus on safety and performance, combined with its growing ecosystem, makes it perfect for modern software development.</p><h2>The Beginning</h2><p>I started learning Rust because I was curious about systems programming and wanted to understand how to write safe, fast code...</p><h2>Key Insights</h2><ul><li>Ownership system takes time to understand but is incredibly powerful</li><li>The compiler is your friend, not your enemy</li><li>The community is welcoming and helpful</li><li>Performance benefits are real and measurable</li></ul>")),
                ContentBlock::Prose(String::from(r#"<iframe width="560" height="315" src="https://www.youtube.com/embed/5C_HPTJg5PU" title="Why You Should Learn Rust" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>"#)),
            ],
            excerpt: String::from("Personal reflections on learning Rust and building with this amazing language."),
            date: String::from("2025-08-18"),
            category: String::from("personal"),
            tags: vec![
                Tag { name: String::from("rust"), color: Rgba { r: 222, g: 82, b: 70, a: 1.0 } },
                Tag { name: String::from("learning"), color: Rgba { r: 107, g: 114, b: 128, a: 1.0 } },
                Tag { name: String::from("personal"), color: Rgba { r: 255, g: 107, b: 53, a: 1.0 } },
            ],
            reading_time: 6,
            featured: false,
        },
        BlogPost {
            id: 4,
            title: String::from("Exploring Markdown in Dioxus"),
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

![Dioxus Logo](/images/dioxus-logo.png "Dioxus Logo")

<iframe width="560" height="315" src="https://www.youtube.com/watch?v=W1ylgNSsJZg&list=RDW1ylgNSsJZg&start_radio=1&t=7s" title="Dioxus: Rust Web Framework" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

That's it! More content can follow.
"#),
            excerpt: String::from("A demonstration of Markdown post support in your Dioxus blog."),
            date: String::from("2025-08-21"),
            category: String::from(""),
            tags: vec![
                Tag { name: String::from("markdown"), color: Rgba { r: 59, g: 130, b: 246, a: 1.0 } },
                Tag { name: String::from("dioxus"), color: Rgba { r: 16, g: 185, b: 129, a: 1.0 } },
                Tag { name: String::from("rust"), color: Rgba { r: 222, g: 82, b: 70, a: 1.0 } },
            ],
            reading_time: 4,
            featured: true,
        },
    ]
}