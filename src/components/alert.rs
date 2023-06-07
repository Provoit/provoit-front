use dioxus::{prelude::*, core::AttributeValue};

/// Alert severity
#[derive(PartialEq, Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Success,
}

impl<'a> IntoAttributeValue<'a> for Severity {
    fn into_value(self, _bump: &'a dioxus::core::exports::bumpalo::Bump) -> dioxus::core::AttributeValue<'a> {
        AttributeValue::Text(match self {
            Self::Error => "alert-error",
            Self::Warning => "alert-warning",
            Self::Info => "alert-info",
            Self::Success => "alert-success"
        })
    }
}

#[derive(Props)]
pub struct AlertProps<'a> {
    /// Alert severity
    severity: Severity,
    /// Element to display in the alert box
    children: Element<'a>,
}

/// Alert component.
/// Similar to what [material UI can offer](https://mui.com/material-ui/react-alert/).
pub fn Alert<'a>(cx: Scope<'a, AlertProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div { class: cx.props.severity, &cx.props.children }
    })
}
