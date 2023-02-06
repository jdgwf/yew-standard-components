use super::input_label::{InputLabel, _InputLabelProps::label};
use gloo_console::error;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputNumberProps {
    #[prop_or_default]
    pub onchange: Callback<f32>,

    #[prop_or_default]
    pub input_type: String,

    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub value: f32,

    #[prop_or_default]
    pub description: String,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub inline: bool,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub label_class: String,

    #[prop_or_default]
    pub input_class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub min: Option<String>,

    #[prop_or_default]
    pub max: Option<String>,

    #[prop_or_default]
    pub step: Option<String>,
}

pub enum InputNumberMessage {
    OnChange(String),
}

pub struct InputNumber;

impl Component for InputNumber {
    type Message = InputNumberMessage;
    type Properties = InputNumberProps;

    fn create(_ctx: &Context<Self>) -> Self {
        InputNumber {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: InputNumberMessage) -> bool {
        match msg {
            InputNumberMessage::OnChange(new_value) => {

                let nv_res = new_value.parse::<f32>();
                match nv_res {
                    Ok( nv ) => {
                        ctx.props().onchange.emit(nv);
                    }
                    Err (err ) => {
                        error!( format!("Cannot format f32! '{}' - {}", new_value, err) );
                    }
                }

                false
            }

        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut description = html!(<></>);
        if ctx.props().description.to_owned() != "" {
            description = html!(
                <div class={"small-text"}>
                { ctx.props().description.to_owned() }
                </div>
            );
        }

        let mut val = ctx.props().value.to_string();

        let mut drop_down_version = false;

        match &ctx.props().step {
            Some( step ) => {
                if step == "1" {
                    val = ( ctx.props().value.round() as i32 ).to_string();

                    if ctx.props().min != None && ctx.props().max != None {
                        drop_down_version = true;
                    }
                }

            }
            None => {
            }
        }

        if drop_down_version && !ctx.props().readonly {
            let onchange_select = ctx.link().callback(move |event: Event| {
                event.prevent_default();

                let input: HtmlSelectElement = event.target_unchecked_into();

                InputNumberMessage::OnChange(input.value())

            });

            let min_str = ctx.props().min.clone();
            let max_str = ctx.props().max.clone();

            let min = min_str.unwrap_or("0".to_owned()).parse::<i32>().unwrap_or(0);
            let max = max_str.unwrap_or("0".to_owned()).parse::<i32>().unwrap_or(0) + 1;
            let num_val = val.parse::<i32>().unwrap_or(0);
            let mut label_class = ctx.props().label_class.to_owned();

            if ctx.props().inline {
                label_class = label_class + &" inline";
            }
            html! {
                <label
                    class={label_class.to_owned()}
                    title={ctx.props().title.to_owned()}
                >
                    <InputLabel
                        label={ctx.props().label.to_owned()}
                        inline={ctx.props().inline}
                    />

                    {description}

                    <select
                        class={ctx.props().input_class.to_owned()}
                        readonly={ctx.props().readonly}
                        value={val}
                        onchange={onchange_select}
                    >
                        {(min..max).map( |count: i32 |  {
                            if count == num_val {
                                return html!{<option selected={true} value={count.to_string()}>{count.to_string()}</option>}
                            } else {
                                return html!{<option value={count.to_string()}>{count.to_string()}</option>}
                            }
                        }).collect::<Html>()}
                    </select>

                    { for ctx.props().children.iter() }
                </label>
            }
        } else {
            let onchange_input = ctx.link().callback(|event: InputEvent| {
                event.prevent_default();
                let input: HtmlInputElement = event.target_unchecked_into();
                InputNumberMessage::OnChange(input.value())
            });
            html! {
                <label
                    class={ctx.props().label_class.to_owned()}
                    title={ctx.props().title.to_owned()}
                >

                    <InputLabel
                        label={ctx.props().label.to_owned()}
                        inline={ctx.props().inline}
                    />

                    {description}

                    <input
                        class={ctx.props().input_class.to_owned()}
                        placeholder={ctx.props().placeholder.to_owned()}
                        type={"number"}
                        min={ctx.props().min.clone()}
                        max={ctx.props().max.clone()}
                        step={ctx.props().step.clone()}
                        readonly={ctx.props().readonly}
                        value={val}
                        oninput={onchange_input}
                    />

                    { for ctx.props().children.iter() }
                </label>
            }
        }
    }
}
