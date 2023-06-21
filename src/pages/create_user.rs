use alert::Alert;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaSquarePlus;
use dioxus_free_icons::Icon;
use provoit_types::models::{users::NewUser, vehicles::NewVehicle};
use reqwest::StatusCode;

use crate::{
    components::{alert, AddVehicle},
    utils::request::post,
};

pub fn CreateUserPage(cx: Scope) -> Element {
    let loading = use_state(cx, || false);
    let error = use_state(cx, || None);

    let editing_vehicle = use_state(cx, || false);
    let vehicles = use_ref(cx, Vec::<NewVehicle>::new);

    let on_submit = |event: FormEvent| {
        let loading = loading.clone();
        let error = error.clone();
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
            let res = post("/users", &user).await;

            match res {
                Ok(r) if r.status() == StatusCode::CREATED => error.set(None),
                _ => error.set(Some("Erreur lors de la connexion, veuillez réessayer.")),
            }
            loading.set(false)
        })
    };

    let on_submit_vehicle = |v: NewVehicle| {
        vehicles.with_mut(|list| list.push(v));
        editing_vehicle.set(false);
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
                if *editing_vehicle.current() {
                    rsx! {
                        AddVehicle { onsubmit: on_submit_vehicle, oncancel: |_| editing_vehicle.set(false) }
                    }
                } else {
                    rsx! {
                        div { text_align: "center",
                            p { strong { "Ajouter un véhicule" } }
                            span { onclick: |_| editing_vehicle.set(true), Icon { width: 48, height: 48, icon: FaSquarePlus } }
                        }
                    }
                }
                button { r#type: "submit", "aria-busy": *loading.current(), "Créer un compte" }
            }
            (*error.current()).map(|err| rsx!(Alert { severity: alert::Severity::Error, err }))
        }
    ))
}
