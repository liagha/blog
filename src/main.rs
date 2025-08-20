mod app;
mod routes;
mod components;
mod pages;
mod data;

use dioxus::prelude::*;
use app::App;

fn main() {
    launch(App);
}