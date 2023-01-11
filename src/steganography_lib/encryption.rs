use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCryptError};

pub fn encrypt_if_needed(message: String, password: Option<String>) -> String {
    match password {
        Some(p) => return encrypt(message, p),
        None => message,
    }
}

pub fn decrypt_if_needed(message: String, password: Option<String>) -> Result<String, MagicCryptError> {
    match password {
        Some(p) => return decrypt(message, p),
        None => Ok(message),
    }
}

fn encrypt(message: String, password: String) -> String {
    let mc = new_magic_crypt!(password, 256);
    mc.encrypt_str_to_base64(message)
}

fn decrypt(message: String, password: String) -> Result<String, MagicCryptError> {
    let mc = new_magic_crypt!(password, 256);
    mc.decrypt_base64_to_string(&message)
}
