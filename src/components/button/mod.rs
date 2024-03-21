use leptos::*;

#[component]
pub fn Button(
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! { <button class="btn">{children.map(|c| c())}</button> }
}