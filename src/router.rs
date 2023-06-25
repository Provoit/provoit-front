use dioxus::prelude::*;
use dioxus_router::{Redirect, Route};

use crate::{
    components::Navbar,
    pages::{CreateUserPage, LoginPage, PageNotFound, ProfilePage}, auth::Auth,
};

pub fn Router(cx: Scope) -> Element {
    let auth = use_shared_state::<Auth>(cx).unwrap();

    cx.render(rsx! {
        dioxus_router::Router { 
            Navbar {}
            Route { to: "/login", LoginPage {} }
            Route { to: "/user/create", CreateUserPage {} }
            Route { to: "/profile", ProfilePage {} }
            Route { to: "", PageNotFound {} }
            if auth.read().user.is_none() {
                rsx!(Redirect { to: "/login" })
            } else {
                rsx!(Redirect { to: "/trip/search" })
            }
        }
    })
}
