use yew::prelude::*;

use crate::ui::nbsp::Nbsp;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub close_cancel_callback: Callback<bool>,

    #[prop_or_default]
    pub add_callback: Option<Callback<bool>>,

    #[prop_or_default]
    pub save_callback: Option<Callback<bool>>,

    #[prop_or_default]
    pub save_as_new_callback: Option<Callback<bool>>,

    #[prop_or_default]
    pub add_label: Option<String>,

    #[prop_or_default]
    pub save_label: Option<String>,

    #[prop_or_default]
    pub save_as_new_label: Option<String>,

}

#[function_component]
pub fn StandardFormSaveButtons( props: &Props) -> Html {

    let mut close_cancel_label = "Close".to_owned();

    let mut save_label = "Save".to_owned();
    let mut save_as_new_label = "Save as New".to_owned();
    let mut add_label = "Add".to_owned();

    match &props.add_label {
        Some(ov)=> { add_label = ov.to_owned() }
        None => {}
    }
    match &props.save_as_new_label {
        Some(ov)=> { save_as_new_label = ov.to_owned() }
        None => {}
    }
    match &props.save_label {
        Some(ov)=> { save_label = ov.to_owned() }
        None => {}
    }

    let mut add_button = html!{<></>};
    let mut save_button = html!{<></>};
    let mut save_as_new_button = html!{<></>};

    let close_cancel_callback = props.close_cancel_callback.clone();

    match &props.add_callback {
        Some( cb ) => {
            let the_callback = cb.clone();
            close_cancel_label = "Cancel".to_owned();
            add_button = html!{
                <button
                    class="btn btn-success"
                    onclick={move |_e | {
                        the_callback.emit(true);
                    }}
                >
                    <i class="fa fa-plus" /><Nbsp />{add_label}
                </button>
            };
        }
        None => {

        }
    }
    match &props.save_callback {
        Some( cb ) => {
            let the_callback = cb.clone();
            close_cancel_label = "Cancel".to_owned();
            save_button = html!{
                <button
                    class="btn btn-success"
                    onclick={move |_e | {
                        the_callback.emit(true);
                    }}
                >
                    <i class="fa fa-floppy-disk" /><Nbsp />{save_label}
                </button>
            };
        }
        None => {

        }
    }
    match &props.save_as_new_callback {
        Some( cb ) => {
            let the_callback = cb.clone();
            save_as_new_button = html!{
                <button
                    class="btn btn-secondary"
                    onclick={move |_e | {
                        the_callback.emit(true);
                    }}
                >
                    <i class="fa fa-floppy-disk" /><Nbsp />{save_as_new_label}
                </button>
            };
        }
        None => {

        }
    }

    let close_button = html!{
        <button
            class="btn btn-secondary"
            onclick={move |_e | {
                close_cancel_callback.emit(true);
            }}
        >
            <i class="fa fa-cancel" /><Nbsp />{close_cancel_label}
        </button>
    };

    html! {
        <div class="text-right">
            <div class="pull-left">
                {save_as_new_button}
            </div>

            {close_button}{save_button}{add_button}
        </div>
    }
}