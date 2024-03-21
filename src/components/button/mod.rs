use leptos::*;

#[component]
pub fn Button(
    #[prop(optional)]
    children: Option<Children>,
    #[prop(optional)]
    variant: Option<ButtonVariant>,
    #[prop(optional)]
    outline: Option<bool>
) -> impl IntoView {
    let mut class = "btn ".to_string();
    match variant {
        None => {}
        Some(v) => {
            add_variant_class(v, &mut class, outline)
        }
    }
    view! { <button class=class>{children.map(|c| c())}</button> }
}

pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link
}

fn add_variant_class(variant: ButtonVariant, class: &mut String, outline: Option<bool>){
    match variant {
        ButtonVariant::Primary => {
            class.push_str(push_class("primary", outline).as_str())
        }
        ButtonVariant::Secondary => {
            class.push_str(push_class("secondary", outline).as_str())
        }
        ButtonVariant::Success => {
            class.push_str(push_class("success", outline).as_str())
        }
        ButtonVariant::Danger => {
            class.push_str(push_class("danger", outline).as_str())
        }
        ButtonVariant::Warning => {
            class.push_str(push_class("warning", outline).as_str())
        }
        ButtonVariant::Info => {
            class.push_str(push_class("info", outline).as_str())
        }
        ButtonVariant::Light => {
            class.push_str(push_class("light", outline).as_str())
        }
        ButtonVariant::Dark => {
            class.push_str(push_class("dark", outline).as_str())
        }
        ButtonVariant::Link => {
            class.push_str(push_class("link", outline).as_str())
        }
    };
}

fn push_class(class: &str, outline: Option<bool>) -> String{
    let outline_str = match outline {
        Some(true) => "-outline",
        _ => "",
    };
    format!("btn{}-{}", outline_str, class)
}