use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{
        FaArrowRight, FaCalendarDays, FaCarSide, FaCartFlatbedSuitcase, FaMoneyBillWave, FaPerson,
    },
    Icon,
};
use provoit_types::models::{timings::Timing, trips::Trip, vehicles::Vehicle};

use crate::{
    components::alert::{Alert, Severity},
    hooks::use_token,
    utils::request,
};

#[derive(Props, PartialEq)]
pub struct TripCardProps {
    trip: Trip,
}

pub fn TripCard(cx: Scope<TripCardProps>) -> Element {
    let token = use_token(cx);
    let vehicle = use_future(
        cx,
        &(cx.props.trip.id_vehicle, token.clone()),
        |(id_vec, token)| async move {
            request::get(format!("/vehicles/{id_vec}").as_str(), token)
                .await?
                .json::<Vehicle>()
                .await
        },
    );

    let start_timing = use_future(
        cx,
        &(cx.props.trip.id_start_timing, token.clone()),
        |(id_start_timing, token)| async move {
            request::get(format!("/timings/{id_start_timing}").as_str(), token)
                .await?
                .json::<Timing>()
                .await
        },
    );
    let end_timing = use_future(
        cx,
        &(cx.props.trip.id_end_timing, token.clone()),
        |(id_end_timing, token)| async move {
            request::get(format!("/timings/{id_end_timing}").as_str(), token)
                .await?
                .json::<Timing>()
                .await
        },
    );

    cx.render(rsx! {
        article {
            header {
                div { display: "flex", justify_content: "space-between",
                    h2 {
                        "{cx.props.trip.start}"
                        " "
                        Icon { icon: FaArrowRight }
                        " "
                        "{cx.props.trip.end}"
                    }
                    // span to avoid full size button
                    span { button { role: "button", class: "outline", "Rejoindre" } }
                }
                match (start_timing.value(), end_timing.value()) {
                    (Some(Ok(st)), Some(Ok(et))) => rsx!(
                        div {
                            "{st.time}"
                            " "
                            Icon { icon: FaArrowRight }
                            " "
                            "{et.time}"
                        }
                    ),
                    _ => rsx!(Alert { severity: Severity::Error, "Erreur lors de la récupération des horaires" })
                }
            }
            div { class: "grid",
                div {
                    p {
                        Icon { icon: FaMoneyBillWave }
                        " "
                        "{cx.props.trip.price} €"
                    }
                    p {
                        Icon { icon: FaPerson }
                        " "
                        "{cx.props.trip.max_people} places"
                    }
                }
                div { aria_busy: vehicle.value().is_none(),
                    match vehicle.value() {
                        Some(Ok(vec)) => rsx!(
                            p {
                                Icon { icon: FaCarSide }
                                " "
                                "{vec.name}"
                            }
                            p {
                                Icon { icon: FaCartFlatbedSuitcase }
                                " "
                                span { "data-tooltip": "Taille du coffre", "{vec.trunk_size_L} litres" }
                            }
                            p {
                                Icon { icon: FaCalendarDays }
                                " "
                                span { "data-tooltip": "Année de la voiture", "{vec.year}" }
                            }
                        ),
                        Some(Err(_)) => rsx!(Alert { severity: Severity::Error, "Erreur lors de la récupération du véhicule" }),
                        _ => rsx!("")
                    }
                }
            }
        }
    })
}
