// use gloo_console::log;
// use block_rule;
// use core_rule;
// use inline_rule;
// use markdown_it;

// This one is not as good as markdown_it,
// but markdown_it doesn't compile on my mac due to a dependency compile error
use markdown_to_html;

use web_sys::{Element, Node};
use yew::{function_component, Html, Properties};
#[derive(Properties, PartialEq)]
pub struct Props {
    pub markdown: String,
}

#[function_component]
pub fn MarkdownView(props: &Props) -> Html {
    // add a final newline so the parser lib doesn't gripe.
    let markdown_text = props.markdown.to_owned() + &"\n";

    // let md = &mut markdown_it::MarkdownIt::new();

    // add commonmark syntax, you almost always want to do that
    // markdown_it::plugins::cmark::add(md);

    // add custom three rules described above
    // inline_rule::add(md);
    // block_rule::add(md);
    // core_rule::add(md);

    // let inner_html = md.parse(&markdown_text).render();

    // Reverted back to markdown_to_html for now
    let inner_html = markdown_to_html::markdown( &markdown_text ) ;

    let div: Element = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&inner_html);
    let node: Node = div.into();

    Html::VRef(node)
}
