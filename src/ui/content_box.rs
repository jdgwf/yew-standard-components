use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContentBoxProps {
    #[prop_or_default]
    pub label_html: Option<Html>,

    #[prop_or_default]
    pub label: Option<String>,

    #[prop_or_default]
    pub class: Option<String>,

    #[prop_or_default]
    pub children: Children,
}

pub enum ContentBoxMessage {}

pub struct ContentBox;

impl Component for ContentBox {
    type Message = ContentBoxMessage;
    type Properties = ContentBoxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ContentBox {}
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {
        let mut label = "".to_string();
        match &ctx.props().label {
            Some(label_val) => {
                if !label_val.is_empty() {
                    label = label_val.to_owned();
                }
            }
            None => {}
        }

        let mut class = "content-box".to_owned();
        match &ctx.props().class {
            Some(class_val) => {
                if class_val.is_empty() {
                    class = "content-box".to_owned();
                } else {
                    class = "content-box ".to_owned() + &class_val;
                }
            }
            None => {}
        }

        let label_html = ctx.props().label_html.clone();

        html! {
            <div
                class={class}
            >
                if !label.is_empty() {
                    <div class={"content-box-label"}>{label}</div>
                }
                if label_html != None {
                    <div class={"content-box-label"}>{label_html.unwrap()}</div>

                }
                <div class="content-box-contents">
                    { for ctx.props().children.iter() }
                </div>
            </div>
        }
    }
}
