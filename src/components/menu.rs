use dioxus::prelude::*;
use dioxus_router::Link;

use crate::utils::request;

#[derive(Props)]
pub struct MenuProps<'a> {
    onnavigate: EventHandler<'a>,
}

pub fn Menu<'a>(cx: Scope<'a, MenuProps<'a>>) -> Element<'a> {
    let disconnect = |_: MouseEvent| {
        cx.spawn(async move {
            let _ = request::post("/logout", "").await;
        });
    };

    cx.render(rsx!(
        aside {
            nav {
                ul {
                    li {
                        Link { to: "/profile", onclick: |_| cx.props.onnavigate.call(()), "Profile" }
                    }
                    li {
                        Link { to: "/traject/create", onclick: |_| cx.props.onnavigate.call(()), "Créer un trajet" }
                    }
                    li {
                        Link { to: "/traject/search", onclick: |_| cx.props.onnavigate.call(()), "Rechercher un trajet" }
                    }
                    li { a { onclick: disconnect, href: "#", "Déconnexion" } }
                }
            }
        }
    ))
}
