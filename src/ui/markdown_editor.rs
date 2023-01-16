use gloo_console::log;
use yew::prelude::*;
use super::{input_label::InputLabel, nbsp::Nbsp};
use web_sys::{HtmlInputElement};
use crate::ui::markdown_view::MarkdownView;
use substring::Substring;

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

    #[prop_or_default]
    pub starting_height: u32,
}

pub enum MarkdownEditorMessage {
    OnChange(InputEvent),
    SetTab(String),
    InsertTab(KeyboardEvent),
    DoNothing(),
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
            MarkdownEditorMessage::OnChange( event ) => {

                let input: HtmlInputElement = event.target_unchecked_into();

                ctx.props().onchange.emit( input.value() );
                return false;
            }

            MarkdownEditorMessage::DoNothing() => {
                return false;
            }
            MarkdownEditorMessage::InsertTab(event) => {

                let input: HtmlInputElement = event.target_unchecked_into();

                let mut current_election_start: u32 = 0;
                let mut current_election_end: u32 = 0;
                match input.selection_start() {
                    Ok( val ) => {
                        current_election_start = val.unwrap();
                    }
                    Err( _err ) => {}
                }

                match input.selection_end() {
                    Ok( val ) => {
                        current_election_end = val.unwrap();
                    }
                    Err( _err ) => {}
                }

                let end: usize = current_election_end.try_into().unwrap();
                let start: usize = current_election_start.try_into().unwrap();

                // log!("current_election_start", current_election_start);
                // log!("current_election_end", current_election_end);

                // log!("start", start);
                // log!("end", end);

                // ctx.props().onchange.emit( new_value + "\t" );
                let value = ctx.props().value.clone();
                let first = value.substring(0, start).to_owned();
                let last = value.substring(end, value.len() ).to_owned();
                log!("first", &first);
                log!("last", &last);
                let new_value = first + &"\t" + &last;

                log!("new_value", &new_value);

                ctx.props().onchange.emit( new_value );
                // let _ = input. /set_selection_start( Some(current_election_start + 1) );
                return false;
            }
            MarkdownEditorMessage::SetTab( new_value ) => {
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
                return true;
            }
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let mut edit_style = "height: 100px".to_owned();
        let mut view_style = "min-height: 100px".to_owned();

        if ctx.props().starting_height > 100 {
            edit_style = "height: ".to_owned() + &ctx.props().starting_height.to_string() + &"px";
            view_style = "min-height: ".to_owned() + &ctx.props().starting_height.to_string() + &"px";
        }

        let onchange = ctx.link().callback(
            |event: InputEvent| {

                MarkdownEditorMessage::OnChange(event)
            }
        );

        let onkeydown= ctx.link().callback(
            |event: KeyboardEvent| {
                if event.key_code() == 9 {
                    log!("istab");
                    event.prevent_default();
                    return MarkdownEditorMessage::InsertTab(event);
                }
                return MarkdownEditorMessage::DoNothing();

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

                if ctx.props().readonly {
                    <div style={view_style} class="markdown-edit-preview">
                    <MarkdownView markdown={ctx.props().value.to_owned()} />
                </div>
                } else {
                    <>
                    <div class="flex">

                        <div>
                            <button
                                type="button"
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
                                type="button"
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
                            onkeydown={onkeydown}
                            style={edit_style}
                            value={ctx.props().value.to_owned()}
                        />
                    } else {
                        <div style={view_style} class="markdown-edit-preview">
                            <MarkdownView markdown={ctx.props().value.to_owned()} />
                        </div>
                    }
                    </>
                }
                { for ctx.props().children.iter() }
            </label>
        }
    }
}
