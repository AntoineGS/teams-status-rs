pub fn bool_to_str(bool: bool) -> String {
    return if bool {
        "on".to_string()
    } else {
        "off".to_string()
    };
}

pub fn set_if_empty(value: &mut String, default_if_empty: &str) {
    if value.is_empty() {
        *value = default_if_empty.to_string();
    }
}
