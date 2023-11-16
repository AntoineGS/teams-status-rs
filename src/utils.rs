// Tried a couple of different encoders like age and simple_crypt and I was unable to convert
// the encrypted data into utf8 to write into the ini file, base64 is better than nothing.. tbc
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

const CRYPTO_KEY: &str = env!("CRYPTO_KEY");
const ENCODED_PREFIX: &str = "en//";
pub fn bool_to_str(bool: bool) -> String {
    return if bool {
        "on".to_string()
    } else {
        "off".to_string()
    };
}

pub fn encrypt(value: &str) -> String {
    let mc = new_magic_crypt!(CRYPTO_KEY, 256);

    format!(
        "{prefix}{value}",
        prefix = ENCODED_PREFIX,
        value = mc.encrypt_str_to_base64(value)
    )
}

fn decrypt(value: &str) -> String {
    let mc = new_magic_crypt!(CRYPTO_KEY, 256);
    mc.decrypt_base64_to_string(value).unwrap()
}

pub fn decrypt_if_needed(value: &str) -> String {
    if value.starts_with(ENCODED_PREFIX) {
        decrypt(&value[ENCODED_PREFIX.len()..])
    } else {
        value.to_string()
    }
}
