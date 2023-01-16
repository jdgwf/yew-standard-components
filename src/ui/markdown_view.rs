// use gloo_console::log;
use yew::{function_component, Html, Properties};
use web_sys::{Element, Node};
use markdown_to_html;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub markdown: String,
}

#[function_component]
pub fn MarkdownView( props: &Props) -> Html {

    // add a final newline so the parser lib doesn't gripe.
    let markdown_text = props.markdown.to_owned() + &"\n";

    let inner_html = markdown_to_html::markdown( &markdown_text ) ;

    let div: Element = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html( &inner_html );
    let node: Node = div.into();

    Html::VRef(node)
}