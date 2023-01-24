use yew::{function_component, html, Html};

#[function_component]
pub fn Nbsp() -> Html {
    html! { "\u{00a0}" }
}
