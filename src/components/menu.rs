use dioxus::prelude::*;
use dioxus_router::Link;

use crate::utils::request;

pub fn Menu(cx: Scope) -> Element {
    let disconnect = |_: MouseEvent| {
        cx.spawn(async move {
            let _ = request::post("/logout", "").await;
        });
    };

    cx.render(rsx!(
        ul {
            li {
                Link { to: "/profile", "Profile" }
            }
            li {
                Link { to: "/traject/create", "Créer un trajet" }
            }
            li {
                Link { to: "/traject/search", "Rechercher un trajet" }
            }
            li { a { onclick: disconnect, href: "#", "Déconnexion" } }
        }
    ))
}
