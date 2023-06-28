pub fn get_env_var(key: &str) -> String {
    // I would have liked to convert the "env..." string to be a const but was unable to make
    // it work :(
    let error_msg = &*format!("{} env variable must be set", key);
    let env_var = std::env::var(key).expect(error_msg);

    if str::trim(&env_var) == "" {
        panic!("{}", error_msg);
    }
    env_var
}

pub fn bool_to_str(bool: bool) -> String {
    return if bool {
        "on".to_string()
    } else {
        "off".to_string()
    };
}
