use dioxus::prelude::*;

pub fn SearchTripPage(cx: Scope) -> Element {
    cx.render(rsx! { h1 { text_align: "center", "Chercher un trajet" } })
}
