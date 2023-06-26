use chrono::{NaiveDate, NaiveTime};
use dioxus::prelude::*;
use provoit_types::models::{creation::CreateTrip, timings::NewTiming, vehicles::Vehicle};

use crate::utils::request;

#[derive(Props, PartialEq)]
pub struct CreateTripPageProps {
    id_user: u64,
}

pub fn CreateTripPage(cx: Scope<CreateTripPageProps>) -> Element {
    let loading = use_state(cx, || false);
    let recurring_traject = use_state(cx, || false);

    let vehicles = use_future(cx, &(cx.props.id_user,), |(id_user,)| async move {
        request::get(format!("/users/{id_user}/vehicles").as_str())
            .await?
            .json::<Vec<Vehicle>>()
            .await
    });

    let on_submit = |e: FormEvent| {
        let data = CreateTrip {
            trip: e.values.clone().into(),
            start_timing: NewTiming {
                date: e.values.get("start_date").map(|d| {
                    NaiveDate::parse_from_str(d, "%Y-%m-%d").expect("Date de début invalide")
                }),
                time: NaiveTime::parse_from_str(
                    e.values
                        .get("start_time")
                        .expect("L'heure de départ est requise"),
                    "%H:%M",
                )
                .expect("Heure de départ invalide"),
                id_day: e
                    .values
                    .get("id_day")
                    .map(|v| v.parse().expect("Jour incorrecte")),
            },
            end_timing: NewTiming {
                date: e.values.get("end_date").map(|d| {
                    NaiveDate::parse_from_str(d, "%Y-%m-%d").expect("Date d'arriver invalide")
                }),
                time: NaiveTime::parse_from_str(
                    e.values
                        .get("end_time")
                        .expect("L'heure d'arriver est requise"),
                    "%H:%M",
                )
                .expect("Heure d'arriver invalide"),
                id_day: None,
            },
            road_types: Vec::new(),
        };

        cx.spawn(async move {
            let _ = request::post("/trips", &data).await;
        })
    };

    cx.render(rsx! {
        main { class: "container",
            h1 { text_align: "center", "Créer un trajet" }
            form { prevent_default: "onsubmit", onsubmit: on_submit,
                div { class: "grid",
                    label {
                        "Lieux de départ"
                        input { name: "start", required: true }
                    }
                    label {
                        "Lieux d'arriver"
                        input { name: "end", required: true }
                    }
                }
                label {
                    input {
                        r#type: "checkbox",
                        value: "{recurring_traject}",
                        name: "recurring",
                        oninput: |e| recurring_traject.set(e.value.eq("true"))
                    }
                    "Trajet régulier"
                }
                if *recurring_traject.current() {
                    rsx!(
                        div { class: "grid",
                            label {
                                "Jour"
                                select { required: true, name: "id_day",
                                    option { value: "1", "Lundi" }
                                    option { value: "2", "Mardi" }
                                    option { value: "3", "Mercredi" }
                                    option { value: "4", "Jeudi" }
                                    option { value: "5", "Vendredi" }
                                    option { value: "6", "Samedi" }
                                    option { value: "7", "Dimanche" }
                                }
                            }
                            label {
                                "Fréquence"
                                select { required: true, name: "id_frequency",
                                    option { value: "2", "Journalier" }
                                    option { value: "3", "Hebdomadaire" }
                                    option { value: "4", "Mensuel" }
                                }
                            }
                        }
                    )
                } else {
                    rsx!(
                        div { class: "grid",
                            label {
                                "Date de départ"
                                input { r#type: "date", name: "start_date", required: true }
                            }
                            label {
                                "Date d'arriver"
                                input { r#type: "date", name: "end_date", required: true }
                            }
                        }
                    )
                }
                div { class: "grid",
                    label {
                        "Heure de départ"
                        input { r#type: "time", name: "start_time", required: true }
                    }
                    label {
                        "Heure d'arriver"
                        input { r#type: "time", name: "end_time", required: true }
                    }
                }
                hr {}
                label {
                    "Nombre de personnes maximum"
                    input { r#type: "number", required: true, name: "max_people", min: 1, max: 255 }
                }
                label {
                    "Type de route"
                    select {}
                }
                label { "aria-busy": vehicles.value().is_none(),
                    "Sélectionner un véhicule"
                    select { name: "id_vehicle", disabled: vehicles.value().is_none(), required: true,
                        match vehicles.value() {
                            Some(Ok(vecs)) => {
                                rsx!(vecs.iter().map(|v| {
                                 rsx!(option { key: "{v.id}", value: "{v.id}", "{v.name}" })
                                }))
                            },
                            _ => rsx!("")
                        }
                    }
                }
                label {
                    "Prix (en €)"
                    input { r#type: "number", name: "price", min: 0, required: true }
                }
                button { r#type: "submit", "aria-busy": *loading.current(), "Valider" }
            }
        }
    })
}
