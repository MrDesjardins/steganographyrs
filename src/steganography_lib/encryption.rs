use magic_crypt::{new_magic_crypt, MagicCryptError, MagicCryptTrait};

/// Encrypt a message using a password if provided. When not provided, the function
/// returns the message without alteration
///
/// # Arguments
/// message - The message to inject into the image
/// password - The secret used to modify the message before insertion into the image
///
/// # Returns
/// The message that is encrypted if a password is provided
pub fn encrypt_if_needed(message: String, password: Option<String>) -> String {
    match password {
        Some(p) => encrypt(message, p),
        None => message,
    }
}

/// Decrypt a message using the provided password.
///
/// # Arguments
/// message - Message that may or not be encrypted
/// password - The secret to use to decrypt the message. If not provided, the message
/// is returned without alteration
///
/// #Returns
/// The message decrypted, readable by a human
pub fn decrypt_if_needed(
    message: String,
    password: Option<String>,
) -> Result<String, MagicCryptError> {
    match password {
        Some(p) => decrypt(message, p),
        None => Ok(message),
    }
}

/// Encrypt the message with the password using AES256. The result is wrapped in base64.
///
/// # Arguments
/// message - The message to encrypt
/// password - Secret to encrypt the message
///
/// # Returns
/// Encrypted message
fn encrypt(message: String, password: String) -> String {
    let mc = new_magic_crypt!(password, 256);
    mc.encrypt_str_to_base64(message)
}

/// Decrypt the message using the password.
///
/// # Arguments
/// message - Encrypted message
/// password - Secret to decrypt the message
///
/// #Returns
/// Decrypted message or failure result if something is wrong
fn decrypt(message: String, password: String) -> Result<String, MagicCryptError> {
    let mc = new_magic_crypt!(password, 256);
    mc.decrypt_base64_to_string(message)
}
