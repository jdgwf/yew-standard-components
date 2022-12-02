use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContentBoxProps {

    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

}

pub enum ContentBoxMessage {
}

pub struct ContentBox;

impl Component for ContentBox {
    type Message = ContentBoxMessage;
    type Properties = ContentBoxProps;

     fn create(_ctx: &Context<Self>) -> Self {
        ContentBox {

        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let label = ctx.props().label.to_owned();

        let mut class = ctx.props().class.to_owned();
        if class.is_empty() {
            class = "content-box".to_owned();
        } else {
            class = "content-box ".to_owned() + &class;
        }

        html! {
            <div
                class={class}
            >
                if !label.is_empty() {
                    <div class={"content-box-label"}>{label}</div>
                }
                <div class="content-box-contents">
                    { for ctx.props().children.iter() }
                </div>
            </div>
        }
    }
}
