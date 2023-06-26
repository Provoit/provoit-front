use dioxus::prelude::*;
use provoit_types::models::trips::Trip;

use crate::{components::TripCard, hooks::use_token, utils::request};

pub fn SearchTripPage(cx: Scope) -> Element {
    let token = use_token(cx);

    let trips = use_future(cx, &(token,), |(token,)| async move {
        request::get("/trips/search", token)
            .await?
            .json::<Vec<Trip>>()
            .await
    });

    cx.render(rsx! {
        main { class: "container",
            h1 { text_align: "center", aria_busy: trips.value().is_none(), "Chercher un trajet" }
            match trips.value() {
                Some(Ok(trips)) => {
                    rsx!(trips.iter().map(|tr| {
                        rsx!(TripCard { trip: tr.to_owned() })
                    }))
                },
                _ => rsx!("")
            }
        }
    })
}
