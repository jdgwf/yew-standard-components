use yew::prelude::*;
use super::{super::internal::ui::input_label::InputLabel, nbsp::Nbsp};
use web_sys::{HtmlInputElement};
use crate::ui::markdown_view::MarkdownView;

#[derive(Properties, PartialEq)]
pub struct MarkdownEditorProps {

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

}

pub enum MarkdownEditorMessage {
    OnChange(String),
    SetTab(String),
}

pub struct MarkdownEditor {
    edit: bool,
}

impl Component for MarkdownEditor {
    type Message = MarkdownEditorMessage;
    type Properties = MarkdownEditorProps;

     fn create(_ctx: &Context<Self>) -> Self {
        MarkdownEditor {
            edit: true,
        }
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: MarkdownEditorMessage,
    ) -> bool {
        match msg {
            MarkdownEditorMessage::OnChange( new_value ) => {
                // self.value += 1;
                ctx.props().onchange.emit( new_value );
                false
            }
            MarkdownEditorMessage::SetTab( new_value ) => {
                // self.value += 1;
                match new_value.as_str() {
                    "edit" => {
                        self.edit = true;
                    }
                    "view" => {
                        self.edit = false;
                    }
                    _ => {
                        self.edit = true;
                    }
                }
                true
            }
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let onchange = ctx.link().callback(
            |event: InputEvent| {
                let input: HtmlInputElement = event.target_unchecked_into();
                MarkdownEditorMessage::OnChange(input.value())
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

        let mut edit_button_class = "btn btn-sm no-wrap btn-primary";
        let mut view_button_class = "btn btn-sm no-wrap btn-secondary";

        if !self.edit {
            edit_button_class = "btn btn-sm no-wrap btn-secondary";
            view_button_class = "btn btn-sm no-wrap btn-primary";
        }

        let edit_click = ctx.link().callback(MarkdownEditorMessage::SetTab).clone();
        let view_click = ctx.link().callback(MarkdownEditorMessage::SetTab).clone();
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

                <div class="flex">

                    <div>
                        <button
                            class={edit_button_class}
                            onclick={move |_e| {
                                edit_click.emit("edit".to_owned());
                            }}
                        >
                            <i class="fa fa-edit" /><Nbsp />{"Edit"}
                        </button>
                    </div>
                    <div>
                        <button
                            class={view_button_class}
                            onclick={move |_e| {

                                view_click.emit("view".to_owned());
                            }}
                        >
                            <i class="fa fa-eye" /><Nbsp />{"Preview"}
                        </button>
                    </div>
                    <div class="flex-grow-1 small-text">
                        <a href="https://www.markdownguide.org/basic-syntax/" target="markdown">{"Markdown"}</a><Nbsp />{"is enabled on this control. To preview your markdown toggle the Edit/Preview buttons to the left"}
                    </div>
                </div>
                if self.edit {
                    <textarea
                        class={ctx.props().input_class.to_owned()}
                        placeholder={ctx.props().placeholder.to_owned()}
                        readonly={ctx.props().readonly}
                        oninput={onchange}
                        value={ctx.props().value.to_owned()}
                    />
                } else {
                    <div class="markdown-edit-preview">
                        <MarkdownView markdown={ctx.props().value.to_owned()} />
                    </div>
                }
                { for ctx.props().children.iter() }
            </label>
        }
    }
}
