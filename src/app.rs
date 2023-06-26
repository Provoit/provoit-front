use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};

use crate::{
    components::Navbar,
    pages::{CreateTripPage, CreateUserPage, LoginPage, PageNotFound, ProfilePage, SearchTripPage},
};

/// Application entrypoint.
/// Contains all the routes.
pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        Router {
            Navbar {}
            Route { to: "/login", LoginPage {} }
            Route { to: "/user/create", CreateUserPage {} }
            Route { to: "/trip/search", SearchTripPage {} }
            Route { to: "/trip/create", CreateTripPage { id_user: 1 } }
            Route { to: "/profile", ProfilePage {} }
            Route { to: "", PageNotFound {} }
            Redirect { to: "/trip/search" }
        }
    ))
}
