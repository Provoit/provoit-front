use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Menu(cx: Scope) -> Element {
    cx.render(rsx!(
        ul {
            li {
                Link { to: "/profile", "Profile" }
            }
            li {
                Link { to: "/trip/create", "Créer un trajet" }
            }
            li {
                Link { to: "/trip/search", "Rechercher un trajet" }
            }
            li {
                Link { to: "/profile", "Déconnexion" }
            }
        }
    ))
}
