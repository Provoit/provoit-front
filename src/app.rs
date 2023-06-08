use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};

use crate::{
    components::Menu,
    pages::{CreateUserPage, LoginPage, PageNotFound, ProfilePage},
};

/// Application entrypoint.
/// Contains all the routes.
pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        Router {
            Menu {}
            Route { to: "/login", LoginPage {} }
            Route { to: "/user/create", CreateUserPage {} }
            Route { to: "/profile", ProfilePage {} }
            Route { to: "", PageNotFound {} }
            Redirect { to: "/login" }
        }
    ))
}
