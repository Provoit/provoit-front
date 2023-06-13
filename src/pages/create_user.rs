use alert::Alert;
use dioxus::prelude::*;
use reqwest::StatusCode;

use crate::{utils::request::post, components::alert};

pub fn CreateUserPage(cx: Scope) -> Element {
    let loading = use_state(cx, || false);
    let error = use_state(cx, || None);

    let on_submit = |event: FormEvent| {
        let loading = loading.clone();
        let error = error.clone();
        loading.set(true);

        cx.spawn(async move {
            let res = post("/users", &event.values).await;

            match res {
                Ok(r) if r.status() == StatusCode::CREATED => error.set(None),
                _ => error.set(Some("Erreur lors de la connexion, veuillez réessayer.")),
            }
        })
    };

    cx.render(rsx!(
        main { class: "container",
            header { h1 { "Créer un compte" } }
            form { prevent_default: "onsubmit", onsubmit: on_submit,
                div { class: "grid",
                    label {
                        "NOM"
                        input { name: "lastname", placeholder: "NOM", required: true }
                    }
                    label {
                        "Prenom"
                        input { name: "firstname", placeholder: "Prenom", required: true }
                    }
                }
                label {
                    "Email"
                    input { name: "mail", placeholder: "Email", required: true }
                }
                label {
                    "Mot de passe"
                    input { r#type: "password", name: "passwd", placeholder: "Mot de passe", required: true }
                }
                label {
                    "Confirmation mot de passe"
                    input {
                        r#type: "password",
                        name: "passwd-confirm",
                        placeholder: "Confirmation mot de passe",
                        required: true
                    }
                }
                p { "Plus" }
                button { r#type: "submit", "aria-busy": *loading.current(), "Continuer" }
            }
            (*error.current()).map(|err| rsx!(Alert { severity: alert::Severity::Error, err }))
        }
    ))
}
