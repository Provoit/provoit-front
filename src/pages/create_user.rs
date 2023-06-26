use alert::Alert;
use dioxus::prelude::*;
use dioxus_router::{use_router, Link};
use provoit_types::models::users::NewUser;
use reqwest::StatusCode;

use crate::components::alert;

pub fn CreateUserPage(cx: Scope) -> Element {
    let loading = use_state(cx, || false);
    let error = use_state(cx, || None);

    let router = use_router(cx);

    let on_submit = |event: FormEvent| {
        let loading = loading.clone();
        let error = error.clone();
        let router = router.clone();
        loading.set(true);

        let user = NewUser {
            lastname: event
                .values
                .get("lastname")
                .unwrap_or(&String::default())
                .to_owned(),
            firstname: event
                .values
                .get("firstname")
                .unwrap_or(&String::default())
                .to_owned(),
            mail: event
                .values
                .get("mail")
                .unwrap_or(&String::default())
                .to_owned(),
            smoker: event.values.get("smoker").eq(&Some(&String::from("on"))),
            passwd: event
                .values
                .get("passwd")
                .unwrap_or(&String::default())
                .to_owned(),
            profile_pic: None,
            id_favorite_vehicle: None,
        };

        cx.spawn(async move {
            let res = reqwest::Client::new()
                .post("/users")
                .json(&user)
                .send()
                .await;

            match res {
                Ok(r) if r.status() == StatusCode::CREATED => router.navigate_to("/login"),
                _ => error.set(Some("Erreur lors de la connexion, veuillez réessayer.")),
            }
            loading.set(false);
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
                    input { r#type: "email", name: "mail", placeholder: "Email", required: true }
                }
                label {
                    input { r#type: "checkbox", name: "smoker" }
                    "Fumeur"
                }
                hr {}
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
                button { r#type: "submit", "aria-busy": *loading.current(), "Créer un compte" }
            }
            "Déjà un compte ? "
            Link { to: "/login", "Se connecter" }
            (*error.current()).map(|err| rsx!(Alert { severity: alert::Severity::Error, err }))
        }
    ))
}
