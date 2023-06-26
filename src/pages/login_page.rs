use dioxus::prelude::*;
use dioxus_router::Link;
use reqwest::StatusCode;

use crate::{
    auth::Auth,
    components::{alert, Alert},
};

pub fn LoginPage(cx: Scope) -> Element {
    let loading = use_state(cx, || false);
    let error = use_state(cx, || None);

    let auth = use_shared_state::<Auth>(cx).unwrap();

    let on_submit = |event: FormEvent| {
        let loading = loading.clone();
        let error = error.clone();
        let auth = auth.clone();
        loading.set(true);

        cx.spawn(async move {
            let res = reqwest::Client::new()
                .post("http://localhost:8000/login")
                .json(&event.values)
                .send()
                .await;

            match res {
                Ok(r) if r.status() == StatusCode::OK => {
                    error.set(None);
                    let data = r.json::<Auth>().await.expect("Invalid json");
                    auth.write().user = data.user;
                    auth.write().token = data.token;
                }
                _ => {
                    error.set(Some("Erreur lors de la connexion, veuillez réessayer."));
                }
            }
            loading.set(false);
        });
    };

    cx.render(rsx! {
        main { class: "container",
            header { text_align: "center",
                img { src: "logo.svg" }
                h5 { "Connectez vous pour continuer" }
            }
            form { prevent_default: "onsubmit", onsubmit: on_submit,
                label {
                    "Mail"
                    input { name: "mail", placeholder: "Email", required: true }
                }
                label {
                    "Mot de passe"
                    input { r#type: "password", name: "passwd", placeholder: "Mot de passe", required: true }
                }
                button { r#type: "submit", "aria-busy": *loading.current(), "Continuer" }
            }
            "Pas de compte ? "
            Link { to: "/user/create", "Créer un compte" }
            (*error.current()).map(|err| rsx!(Alert { severity: alert::Severity::Error, err }))
        }
    })
}
