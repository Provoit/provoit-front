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

            if let Some(user) = &auth.read().user {
                rsx!(
                    Route { to: "/user/create", CreateUserPage {} }
                    Route { to: "/trip/search", SearchTripPage {} }
                    Route { to: "/trip/create", CreateTripPage { id_user: user.id } }
                    Route { to: "/profile", ProfilePage {} }
                    Redirect { to: "/trip/search" }
                )
            } else {
                rsx!(Redirect { to: "/login" })
            }
            Route { to: "", PageNotFound {} }
        }
    })
}
