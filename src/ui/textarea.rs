use yew::prelude::*;
use super::super::internal::ui::input_label::InputLabel;
use web_sys::{HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct TextAreaProps {

    #[prop_or_default]
    pub onchange: Callback<String>,

    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub description: String,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub r#type: String,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub label_class: String,

    #[prop_or_default]
    pub input_class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub starting_height: u32,
}

pub enum TextAreaMessage {
    OnChange(String),
}

pub struct TextArea;

impl Component for TextArea {
    type Message = TextAreaMessage;
    type Properties = TextAreaProps;

     fn create(_ctx: &Context<Self>) -> Self {
        TextArea {

        }
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: TextAreaMessage,
    ) -> bool {
        match msg {
            TextAreaMessage::OnChange( new_value ) => {
                // self.value += 1;
                ctx.props().onchange.emit( new_value );
                false
            }
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let mut style = "height: 100px".to_owned();

        if ctx.props().starting_height > 100 {
            style = "height: ".to_owned() + &ctx.props().starting_height.to_string() + &"px";
        }

        let onchange = ctx.link().callback(
            |event: InputEvent| {
                let input: HtmlInputElement = event.target_unchecked_into();
                TextAreaMessage::OnChange(input.value())
            }
        );

        let mut description = html!(<></>);
        if ctx.props().description.to_owned() != "" {
            description = html!(
                <div class={"small-text"}>
                { ctx.props().description.to_owned() }
                </div>
            );
        }

        html! {
            <label
                class={ctx.props().label_class.to_owned()}
                title={ctx.props().title.to_owned()}
            >
                <InputLabel
                    label={ctx.props().label.to_owned()}
                    inline={false}
                />

                {description}

                <textarea
                    class={ctx.props().input_class.to_owned()}
                    placeholder={ctx.props().placeholder.to_owned()}
                    readonly={ctx.props().readonly}
                    style={style}
                    oninput={onchange}
                    value={ctx.props().value.to_owned()}
                />
                { for ctx.props().children.iter() }
            </label>
        }
    }
}
