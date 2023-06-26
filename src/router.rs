use dioxus::prelude::*;
use dioxus_router::{Redirect, Route};

use crate::{
    auth::Auth,
    components::Navbar,
    pages::{CreateTripPage, CreateUserPage, LoginPage, PageNotFound, ProfilePage, SearchTripPage},
};

pub fn Router(cx: Scope) -> Element {
    let auth = use_shared_state::<Auth>(cx).unwrap();

    cx.render(rsx! {
        dioxus_router::Router {
            Navbar {}
            Route { to: "/login", LoginPage {} }
            Route { to: "/user/create", CreateUserPage {} }
            Route { to: "/trip/search", SearchTripPage {} }
            Route { to: "/trip/create", CreateTripPage {} }
            Route { to: "/profile", ProfilePage {} }
            Route { to: "", PageNotFound {} }

            if auth.read().user.is_some() {
                rsx!(
                    Redirect { to: "/trip/search" }
                )
            } else {
                rsx!(Redirect { to: "/login" })
            }
        }
    })
}
