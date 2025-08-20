use dioxus::prelude::*;
use crate::pages::home::Home;
use crate::pages::post::Post;
use crate::pages::page::Page;
use crate::pages::category::CategoryPage;

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