use leptos::{component, IntoView, view};

#[component]
pub fn InputText() -> impl IntoView {
    view! {
        <input
            type="text"
            class="form-control"
        />
    }
}