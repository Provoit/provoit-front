#![allow(non_snake_case)]
use crate::app::App;

mod app;
mod auth;
mod components;
mod hooks;
mod pages;
mod router;
mod utils;

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
    #[cfg(feature = "logging")]
    console_log::init_with_level(log::Level::Debug).expect("Failed to init logger");
    run();
}
