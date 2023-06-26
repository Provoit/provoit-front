use dioxus::prelude::*;

use crate::auth::Auth;

pub fn use_token(cx: &ScopeState) -> String {
    use_shared_state::<Auth>(cx)
        .expect("Le contexte d'authentification n'est pas défini")
        .read()
        .token
        .clone()
}
