use super::nbsp::Nbsp;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputLabelProps {
    pub label: String,

    #[prop_or_default]
    pub inline: bool,
}

#[function_component]
pub fn InputLabel(props: &InputLabelProps) -> Html {
    if props.label.to_owned().trim() == "".to_owned() {
        return html! {};
    }

    if props.inline {
        return html! {
            <>
            {props.label.to_owned()}{":"}<Nbsp />
            </>
        };
    } else {
        return html! {
            <>
            {props.label.to_owned()}{":"}<br />
            </>
        };
    }
}
