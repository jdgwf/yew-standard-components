use web_sys;

pub fn set_body_class(
    new_value: String,
    server_side_renderer: bool,
) {
    if server_side_renderer {
        return;
    }
    let document = web_sys::window().unwrap().document().unwrap();

    document.body().unwrap().set_class_name(new_value.as_str());

}