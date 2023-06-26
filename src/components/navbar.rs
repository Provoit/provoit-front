use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaBars, FaUser},
    Icon,
};
use dioxus_router::Link;

use crate::{auth::Auth, components::Menu};

pub fn Navbar(cx: Scope) -> Element {
    let open = use_state(cx, || false);
    let auth = use_shared_state::<Auth>(cx).unwrap();

    cx.render(rsx! {
        nav { class: "container-fluid",
            ul {
                auth.read().user.is_some().then(|| rsx!(
                    li { onclick: |_| open.set(!*open.current()), Icon { icon: FaBars } }
                ))
            }
            ul { li { "Provoit" } }
            ul {
                auth.read().user.is_some().then(|| rsx!(
                    li {
                        Link { to: "/profile", Icon { icon: FaUser } }
                    }
                ))
            }
        }
        dialog { id: "menu-dialog", open: *open.current(),
            article { text_align: "center",
                a { href: "#", class: "close", onclick: |_| open.set(false) }
                Menu { onnavigate: |_| open.set(false) }
            }
        }
    })
}
