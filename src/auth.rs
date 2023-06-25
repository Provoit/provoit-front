use dioxus::prelude::*;

use provoit_types::models::users::User;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Auth {
    pub user: Option<User>,
    pub token: String,
}

#[derive(Props)]
pub struct AuthProviderProps<'a> {
    children: Element<'a>,
}

pub fn AuthProvider<'a>(cx: Scope<'a, AuthProviderProps<'a>>) -> Element<'a> {
    use_shared_state_provider(cx, || Auth {
        user: None,
        token: "".to_owned(),
    });

    cx.render(rsx!(&cx.props.children))
}
