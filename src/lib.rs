/*!
# Steganography

Steganography is a Rust library that inject a message into an image.

The word steganography means to hide something. There is a variety of ways to accomplish steganography. This library relies on the least significant bits.

# How to Use?


# Additional Resource

[Blog Post about using the least significant bits](https://patrickdesjardins.com/blog/what-is-steganography-how-to-hide-text-in-image)
*/

pub mod steganography_lib;
use crate::steganography_lib::function::{add_message_to_image, get_message_from_image};
use crate::steganography_lib::options::SteganographyOption;

pub fn steganography(options: SteganographyOption) -> Option<String> {
    match options {
        SteganographyOption::Encrypt(n) => {
            add_message_to_image(n);
            return None;
        }
        SteganographyOption::Decrypt(n) => {
            let result = get_message_from_image(n);
            match result {
                Ok(s) => return Some(s),
                Err(_e) => return None,
            }
        }
    }
}

#[cfg(test)]
mod steganography {
    use crate::steganography_lib::options::{SteganographyDecryptOption, SteganographyEncryptOption};

    use super::*;

    #[test]
    fn test_steganography_encrypt() {
        let options = SteganographyEncryptOption {
            message: "Test Message".to_string(),
            password: None,
            input_image_path: "testAssets/prestine.png".to_string(),
            output_image_path: "testAssets/delete_me.png".to_string(),
        };
        let result = steganography(SteganographyOption::Encrypt(options));
        assert_eq!(None, result)
    }

    #[test]
    fn test_steganography_encrypt_with_password() {
        let options = SteganographyEncryptOption {
            message: "Test Message".to_string(),
            password: Some("Secret Password Here".to_string()),
            input_image_path: "testAssets/prestine.png".to_string(),
            output_image_path: "testAssets/delete_me.png".to_string(),
        };
        let result = steganography(SteganographyOption::Encrypt(options));
        assert_eq!(None, result)
    }

    #[test]
    fn test_steganography_decrypt() {
        let options = SteganographyDecryptOption {
            input_image_path: "testAssets/out_message_Bye_2.png".to_string(),
            password: None,
        };
        let recovered_message = steganography(SteganographyOption::Decrypt(options)).unwrap();
        assert_eq!("Test Message", recovered_message);
    }

    #[test]
    fn test_steganography_decrypt_with_password() {
        let options = SteganographyDecryptOption {
            input_image_path: "testAssets/out_message_Bye_3.png".to_string(),
            password: Some("Secret Password Here".to_string()),
        };
        let recovered_message = steganography(SteganographyOption::Decrypt(options)).unwrap();
        assert_eq!("Test Message", recovered_message);
    }

    #[test]
    fn test_steganography_decrypt_with_wrong_password() {
        let options = SteganographyDecryptOption {
            input_image_path: "testAssets/out_message_Bye_3.png".to_string(),
            password: Some("Wrong Secret Password Here".to_string()),
        };
        let recovered_message = steganography(SteganographyOption::Decrypt(options));
        assert_eq!(None, recovered_message);
    }
}
