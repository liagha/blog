use {
    dioxus::prelude::*,
    crate::{
        pages::{
            home::Home,
            post::Post,
            page::Page,
            category::CategoryPage,
        }
    },
};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/post/:id")]
    Post { id: i32 },
    #[route("/page/:slug")]
    Page { slug: String },
    #[route("/category/:category")]
    CategoryPage { category: String },
}