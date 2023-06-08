use dioxus::prelude::*;
use dioxus_router::Link;
use reqwest::StatusCode;

use crate::{
    components::{alert, Alert},
    utils::request::post,
};

pub fn LoginPage(cx: Scope) -> Element {
    let loading = use_state(cx, || false);
    let error = use_state(cx, || None);

    let on_submit = |event: FormEvent| {
        let loading = loading.clone();
        let error = error.clone();
        loading.set(true);

        cx.spawn(async move {
            let res = post("/login", &event.values).await;

            match res {
                Ok(r) if r.status() == StatusCode::OK => error.set(None),
                _ => error.set(Some("Erreur lors de la connexion, veuillez réessayer.")),
            }
            loading.set(false);
        });
    };

    cx.render(rsx! {
        main { class: "container",
            h5 { "Connectez vous pour continuer" }
            form { prevent_default: "onsubmit", onsubmit: on_submit,
                label {
                    "Mail"
                    input { name: "mail", placeholder: "Email" }
                }
                label {
                    "Mot de passe"
                    input { r#type: "password", name: "passwd", placeholder: "Mot de passe" }
                }
                button { r#type: "submit", "aria-busy": *loading.current(), "Continuer" }
            }
            "Pas de compte ? "
            Link { to: "/user/create", "Créer un compte" }
            (*error.current()).map(|err| rsx!(Alert { severity: alert::Severity::Error, err }))
        }
    })
}
