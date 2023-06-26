use dioxus::prelude::*;

use crate::{auth::AuthProvider, router::Router};

/// Application entrypoint.
/// Contains all the routes.
pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        AuthProvider { Router {} }
    ))
}
