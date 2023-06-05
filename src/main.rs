#![allow(non_snake_case)]
use crate::app::App;

mod app;
mod components;
mod pages;

/// Run the app for desktop
#[cfg(target_family = "wasm")]
fn run() {
    dioxus_web::launch(App);
}

/// Run the app for the web
#[cfg(not(target_family = "wasm"))]
fn run() {
    use dioxus_desktop::Config;

    dioxus_desktop::launch_cfg(
        App,
        Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="dist/pico.min.css">"#.into()),
    )
}

fn main() {
    run();
}
