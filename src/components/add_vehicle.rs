use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaCarSide;
use dioxus_free_icons::Icon;
use dioxus_router::use_router;

use log::debug;
use provoit_types::models::vehicles::NewVehicle;

#[derive(Props)]
pub struct AddVehicleProps<'a> {
    onsubmit: EventHandler<'a, NewVehicle>,
}

pub fn AddVehicle<'a>(cx: Scope<'a, AddVehicleProps<'a>>) -> Element<'a> {
    let router = use_router(cx);

    let on_submit = |e: FormEvent| {
        debug!("{:?}", e.values);
        let vehicle: NewVehicle = e.values.clone().into();
        cx.props.onsubmit.call(vehicle);
    };

    cx.render(rsx! {
        header { text_align: "center",
            Icon { width: 64, height: 64, icon: FaCarSide }
            h1 { "Ajouter un véhicule" }
        }

        form { prevent_default: "onsubmit", onsubmit: on_submit,
            div { class: "grid",
                label {
                    "Type de véhicule"
                    select { required: true, name: "vehicle_type",
                        option { value: "1", "Berline" }
                        option { value: "2", "Citadine" }
                        option { value: "3", "Utilitaire" }
                        option { value: "4", "Franchisseur" }
                        option { value: "5", "SUV" }
                        option { value: "6", "Coupé" }
                        option { value: "7", "Cabriolet" }
                        option { value: "8", "Break" }
                        option { value: "9", "Pickup" }
                    }
                }
                label {
                    "Année"
                    input { name: "year", required: true, r#type: "number", min: 1950, max: 2024 }
                }
            }
            label {
                "Nom"
                input { name: "name", required: true }
            }
            div { class: "grid",
                label {
                    "Nombre de portes"
                    input { required: true, name: "nb_doors", r#type: "number", min: 1 }
                }
                label {
                    "Nombre de sièges"
                    input { required: true, name: "nb_seats", r#type: "number", min: 2 }
                }
                label {
                    "Taille du coffre (en L)"
                    input { required: true, name: "trunk_size_L", r#type: "number", min: 1 }
                }
            }
            div { class: "grid",
                button { r#type: "button", class: "secondary", onclick: |_| { router.pop_route() }, "Annuler" }
                button { r#type: "submit", "Valider" }
            }
        }
    })
}
