use dioxus::prelude::*;

use crate::{
    auth::AuthProvider,
    router::Router
    components::Navbar,
    pages::{CreateTripPage, CreateUserPage, LoginPage, PageNotFound, ProfilePage, SearchTripPage},
};

/// Application entrypoint.
/// Contains all the routes.
pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        AuthProvider { Router {} }
    ))
}
