mod app;
mod routes;
mod components;
mod pages;
mod data;

// src/main.rs
use dioxus::prelude::*;
use app::App;

fn main() {
    launch(App);
}