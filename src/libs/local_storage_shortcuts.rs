use web_sys;

pub fn clear_local_storage() {
    let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let _ = ls.clear();
}

pub fn get_local_storage_u32(
    ls_name: &str,
    default_value: u32,
) -> u32 {
    let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    let mut return_value = default_value;

    let ls_raw_value = ls.get_item(ls_name);

    match ls_raw_value {
        Ok(ls_value) => {
            match ls_value {
                Some(new_value) => {
                    return_value = new_value.parse().unwrap_or(default_value);
                }
                None => {
                    // keep and set default value
                    set_local_storage_u32(ls_name, default_value);
                }
            }
        }
        Err(_) => {
            // keep and set default value
            set_local_storage_u32(ls_name, default_value);
        }
    }

    return_value
}

pub fn get_local_storage_bool(
    ls_name: &str,
    default_value: bool,
) -> bool {
    let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    let mut return_value = default_value;

    let ls_raw_value = ls.get_item(ls_name);

    match ls_raw_value {
        Ok(ls_value) => {
            match ls_value {
                Some(new_value) => {
                    if new_value == "1" {
                        return_value = true;
                    }
                    if new_value == "0" {
                        return_value = false;
                    }
                }
                None => {
                    // keep and set default value
                    set_local_storage_bool(ls_name, default_value);
                }
            }
        }
        Err(_) => {
            // keep and set default value
            set_local_storage_bool(ls_name, default_value);
        }
    }

    return_value
}

pub fn get_local_storage_string(
    ls_name: &str,
    default_value: String,
) -> String {
    let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    let mut return_value = default_value.to_owned();

    let ls_raw_value = ls.get_item(ls_name);

    match ls_raw_value {
        Ok(ls_value) => {
            match ls_value {
                Some(new_value) => {
                    return_value = new_value.to_owned();
                }
                None => {
                    // keep and set default value
                    set_local_storage_string(ls_name, default_value);
                }
            }
        }
        Err(_) => {
            // keep and set default value
            set_local_storage_string(ls_name, default_value);
        }
    }

    return_value
}

pub fn set_local_storage_string(
    ls_name: &str,
    ls_value: String,
) {
    let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    _ = ls.set_item(ls_name, &ls_value);
}

pub fn set_local_storage_bool(
    ls_name: &str,
    ls_value: bool,
) {
    let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if ls_value {
        _ = ls.set_item(ls_name, "1");
    } else {
        _ = ls.set_item(ls_name, "0");
    }
}

pub fn set_local_storage_u32(
    ls_name: &str,
    ls_value: u32,
) {
    let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    _ = ls.set_item(ls_name, &ls_value.to_string());
}
