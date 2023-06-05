use dioxus::prelude::*;
use dioxus_router::{Route, Router};

use crate::{
    components::Menu,
    pages::{LoginPage, PageNotFound, ProfilePage},
};

/// Application entrypoint.
/// Contains all the routes.
pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        Router {
            Menu {}
            Route { to: "/login", LoginPage {} }
            Route { to: "/profile", ProfilePage {} }
            Route { to: "", PageNotFound {} }
        }
    ))
}
