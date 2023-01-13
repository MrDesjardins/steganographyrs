/*!
# Steganography

Steganography is a Rust library that inject a message into an image.

The word steganography means to hide something. There is a variety of ways to accomplish steganography. This library relies on the least significant bits.

# How to Use the Library?

## Encrypt Text into Image

You can use it without a password. In that case the injection (encryption) of the message inside the color of the image is less secure but take less space and faster to generate.

```rust
use steganographyrs::steganography;
use steganographyrs::options::{SteganographyOption, SteganographyEncryptOption};

let options = SteganographyEncryptOption {
            message: "Test Message".to_string(),
            password: None,
            input_image_path: "testAssets/prestine.png".to_string(),
            output_image_path: "testAssets/image_with_secret_message.png".to_string(),
        };
let result = steganography(SteganographyOption::Encrypt(options));
assert_eq!(None, result)
```

You can add a password as a string to modify the message before insertion into the image.

```rust
use steganographyrs::steganography;
use steganographyrs::options::{SteganographyOption, SteganographyEncryptOption};

let options = SteganographyEncryptOption {
    message: "Test Message".to_string(),
    password: Some("Secret Password Here".to_string()),
    input_image_path: "testAssets/prestine.png".to_string(),
    output_image_path: "testAssets/image_with_secret_message.png".to_string(),
};
let result = steganography(SteganographyOption::Encrypt(options));
assert_eq!(None, result)
```

## Decrypt Text into Image

The opposite operation is to get the hidden message from the image. Similar to encrypting the message, the decryption can be with or without a password.

```rust
use steganographyrs::steganography;
use steganographyrs::options::{SteganographyOption, SteganographyDecryptOption};

let options = SteganographyDecryptOption {
    input_image_path: "testAssets/out_message_Bye_2.png".to_string(),
    password: None,
};
let recovered_message = steganography(SteganographyOption::Decrypt(options)).unwrap();
assert_eq!("Test Message", recovered_message);
```

If the message was encrypted, the same password is required to retrieve the message:

```rust
use steganographyrs::steganography;
use steganographyrs::options::{SteganographyOption, SteganographyDecryptOption};

let options = SteganographyDecryptOption {
    input_image_path: "testAssets/out_message_Bye_3.png".to_string(),
    password: Some("Secret Password Here".to_string()),
};
let recovered_message = steganography(SteganographyOption::Decrypt(options)).unwrap();
assert_eq!("Test Message", recovered_message);
```

# How to Use the CLI?

The crate contains a terminal implementation that take parameters to inject or extract the secret string.

## Hide a String without Encryption in an Image

```sh
steganographyrs -e true -m "My Secret Message" -i testAssets/prestine.png -o out.png
```

## Hide an Encrypted String in an Image

```sh
steganographyrs -e true -p secret -m "My Secret Message" -i testAssets/prestine.png -o out.png
```

# Additional Resource

- [Blog Post about using the least significant bits](https://patrickdesjardins.com/blog/what-is-steganography-how-to-hide-text-in-image)
*/

mod utils;

// Imports
use crate::utils::function::{add_message_to_image, get_message_from_image};
use crate::utils::options::SteganographyOption;

// Re-export for external access
pub use crate::utils::options;

pub fn steganography(options: SteganographyOption) -> Option<String> {
    match options {
        SteganographyOption::Encrypt(n) => {
            add_message_to_image(n);
            None
        }
        SteganographyOption::Decrypt(n) => {
            let result = get_message_from_image(n);
            match result {
                Ok(s) => Some(s),
                Err(_e) => None,
            }
        }
    }
}

#[cfg(test)]
mod steganography {
    use crate::utils::options::{
        SteganographyDecryptOption, SteganographyEncryptOption,
    };

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
