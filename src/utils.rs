pub fn bool_to_str(bool: bool) -> String {
    return if bool {
        "on".to_string()
    } else {
        "off".to_string()
    };
}
