use dioxus::prelude::*;
use dioxus_router::{Redirect, Route};

use crate::{
    components::Navbar,
    pages::{CreateUserPage, LoginPage, PageNotFound, ProfilePage},
};

pub fn Router(cx: Scope) -> Element {
    cx.render(rsx! {
        dioxus_router::Router {
            Navbar {}
            Route { to: "/login", LoginPage {} }
            Route { to: "/user/create", CreateUserPage {} }
            Route { to: "/profile", ProfilePage {} }
            Route { to: "", PageNotFound {} }
            Redirect { to: "/login" }
        }
    })
}
