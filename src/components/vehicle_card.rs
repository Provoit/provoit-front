use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::{FaCarSide, FaStar, FaTrashCan};
use dioxus_free_icons::Icon;
use provoit_types::models::vehicles::Vehicle;

#[derive(Props)]
pub struct VehicleCardProps<'a> {
    vehicle: Vehicle,
    ondelete: EventHandler<'a, &'a Vehicle>,
    onfavorite: EventHandler<'a, &'a Vehicle>,
    favorite: bool,
}

pub fn VehicleCard<'a>(cx: Scope<'a, VehicleCardProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        article { display: "flex", justify_content: "space-between",
            span {
                Icon { width: 32, height: 32, icon: FaCarSide }
                " "
                cx.props.vehicle.name.clone()
            }
            span {
                span { onclick: |_| cx.props.onfavorite.call(&cx.props.vehicle),
                    Icon {
                        width: 32,
                        height: 32,
                        icon: FaStar,
                        fill: if cx.props.favorite { "yellow" } else { "currentColor" }
                    }
                }
                " "
                span { onclick: |_| cx.props.ondelete.call(&cx.props.vehicle),
                    Icon { width: 32, height: 32, icon: FaTrashCan }
                }
            }
        }
    })
}
