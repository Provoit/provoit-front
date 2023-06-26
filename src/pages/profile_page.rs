use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaUser, Icon};
use log::debug;
use provoit_types::models::users::User;
use reqwest::StatusCode;

use crate::{
    auth::Auth,
    components::{alert, Alert},
    hooks::use_token,
    utils::request,
};

pub fn ProfilePage(cx: Scope) -> Element {
    let auth = use_shared_state::<Auth>(cx).unwrap();
    let token = use_token(cx);

    let loading = use_state(cx, || false);
    let error = use_state(cx, || None);

    let user = auth.read().user.clone().unwrap();

    let on_submit = move |e: FormEvent| {
        let user_id = user.id;
        let token = token.clone();
        let loading = loading.clone();
        let error = error.clone();
        let auth = auth.clone();

        loading.set(true);

        cx.spawn(async move {
            let res = request::put(
                format!("/users/{}", user_id).as_str(),
                &e.values,
                token.clone(),
            )
            .await;

            match res {
                Ok(r) if r.status() == StatusCode::OK => {
                    error.set(None);
                    let res = request::get("/users/me", token).await;

                    if let Ok(r) = res {
                        error.set(None);
                        let data = r.json::<User>().await.expect("Invalid json");
                        auth.write().user = Some(data);
                    }
                }
                _ => {
                    error.set(Some("Erreur lors de l'enregistrement, veuillez réessayer."));
                }
            }
            loading.set(false);
        });
    };

    cx.render(rsx!(
        main { class: "container",
            header { text_align: "center", Icon { width: 128, height: 128, icon: FaUser } }
            hr {}
            form { prevent_default: "onsubmit", onsubmit: on_submit,
                div { class: "grid",
                    label {
                        "NOM"
                        input { name: "lastname", value: "{user.lastname}" }
                    }
                    label {
                        "Prénom"
                        input { name: "firstname", value: "{user.firstname}" }
                    }
                }
                label {
                    "Mail"
                    input { r#type: "email", name: "mail", value: "{user.mail}" }
                }
                button { r#type: "submit", "aria-busy": *loading.current(), "Valider les changements" }
            }
            (*error.current()).map(|err| rsx!(Alert { severity: alert::Severity::Error, err }))
        }
    ))
}
