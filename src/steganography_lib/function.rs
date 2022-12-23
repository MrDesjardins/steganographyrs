use image::Pixel;

use super::options::{SteganographyDecryptOption, SteganographyEncryptOption};

const NUMBER_BIT_PER_BYTE: u8 = 8;

const EOF_CHAR: char = 4u8 as char;



pub fn add_message_to_image(options: SteganographyEncryptOption) {
    println!("Encrypt into an image");
}

pub fn get_message_from_image(options: SteganographyDecryptOption) {
    println!("Decrypt from an image");
}


