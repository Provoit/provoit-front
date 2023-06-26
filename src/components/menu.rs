use dioxus::prelude::*;
use dioxus_router::Link;

use crate::{auth::Auth, utils::request};

#[derive(Props)]
pub struct MenuProps<'a> {
    onnavigate: EventHandler<'a>,
}

pub fn Menu<'a>(cx: Scope<'a, MenuProps<'a>>) -> Element<'a> {
    let auth = use_shared_state::<Auth>(cx).unwrap();

    let disconnect = move |_: MouseEvent| {
        let token = auth.read().token.clone();
        let auth = auth.clone();

        cx.spawn(async move {
            let _ = request::post("/logout", "", token).await;
            auth.write().user = None;
            auth.write().token = "".to_owned();
        });

        cx.props.onnavigate.call(());
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
