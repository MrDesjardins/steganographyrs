pub mod steganography_lib;
use crate::steganography_lib::function::{add_message_to_image, get_message_from_image};
use crate::steganography_lib::options::SteganographyOption;

pub fn steganography_rs(options: SteganographyOption) -> () {
    match options {
        SteganographyOption::Encrypt(n) => {
            add_message_to_image(n);
        }
        SteganographyOption::Decrypt(n) => {
            get_message_from_image(n);
        }
    }
}
