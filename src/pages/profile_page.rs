use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaUser, Icon};
use provoit_types::models::{
    users::User,
    vehicles::{NewVehicle, Vehicle},
};
use reqwest::StatusCode;

use crate::{
    auth::Auth,
    components::{alert, AddVehicle, Alert, VehicleCard},
    hooks::use_token,
    utils::request,
};

pub fn ProfilePage(cx: Scope) -> Element {
    let auth = use_shared_state::<Auth>(cx).unwrap();
    let token = use_token(cx);

    let loading = use_state(cx, || false);
    let error = use_state(cx, || None);

    let user = auth.read().user.clone().unwrap();
    let vehicles = use_future(
        cx,
        &(user.id, token.clone()),
        |(id_user, token)| async move {
            request::get(format!("/users/{id_user}/vehicles").as_str(), token)
                .await?
                .json::<Vec<Vehicle>>()
                .await
        },
    );

    let token1 = token.clone();
    let on_submit = move |e: FormEvent| {
        let user_id = user.id;
        let loading = loading.clone();
        let error = error.clone();
        let auth = auth.clone();
        let token1 = token1.clone();

        loading.set(true);

        cx.spawn(async move {
            let res = request::put(
                format!("/users/{}", user_id).as_str(),
                &e.values,
                token1.clone(),
            )
            .await;

            match res {
                Ok(r) if r.status() == StatusCode::OK => {
                    error.set(None);
                    let res = request::get("/users/me", token1).await;

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

    let token1 = token.clone();
    let on_submit_vehicle = move |vec: NewVehicle| {
        let token1 = token1.clone();
        cx.spawn(async move {
            let _ = request::post("/vehicles", &vec, token1).await;
        });
    };

    let on_delete_vehicle = move |vec: &Vehicle| {
        let token = token.clone();
        let vec = vec.clone();

        cx.spawn(async move {
            let _ = request::delete(format!("/vehicles/{}", vec.id).as_str(), token).await;
        });
    };

    let on_favorite_vehicle = |_v: &Vehicle| {};

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
            (*error.current()).map(|err| rsx!(Alert { severity: alert::Severity::Error, err })),

            match vehicles.value() {
                Some(Ok(vehicles)) => {
                    rsx!(
                        vehicles.iter().map(|v| {
                            let on_delete_vehicle = on_delete_vehicle.clone();
                            rsx!(VehicleCard {
                                vehicle: v.clone(),
                                ondelete: on_delete_vehicle,
                                onfavorite: on_favorite_vehicle,
                                favorite: user.id_favorite_vehicle.unwrap_or(0) == v.id
                            })
                        })
                    )
                },
                _ => rsx!("")
            },

            AddVehicle { onsubmit: on_submit_vehicle, oncancel: |_| {} }
        }
    ))
}
