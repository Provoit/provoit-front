use dioxus::prelude::*;
use dioxus_router::Link;

pub fn LoginPage(cx: Scope) -> Element {
    cx.render(rsx!(
        main { class: "container",
            h5 { "Connectez vous pour continuer" }
            form {
                prevent_default: "onsubmit",
                onsubmit: move |event| {
                    println!("Submitted {:?}", event);
                },
                label {
                    "Mail"
                    input { name: "mail", placeholder: "Email" }
                }
                label {
                    "Mot de passe"
                    input { r#type: "password", name: "passwd", placeholder: "Mot de passe" }
                }
                button { r#type: "submit", "Continuer" }
            }
            "Pas de compte ? "
            Link { to: "/user/create", "Créer un compte" }
        }
    ))
}
