use web_sys::{Element, Node};
use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub inner_html: String,
}

#[function_component]
pub fn RawHtml(props: &Props) -> Html {
    let div: Element = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.inner_html.clone());
    let node: Node = div.into();
    Html::VRef(node)
}
