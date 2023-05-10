#![allow(non_snake_case)]
use dioxus::prelude::*;

#[cfg(target_family = "wasm")]
fn run() {
    dioxus_web::launch(App);
}

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

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
            button { "C'est un bouton" }
        }
    ))
}
