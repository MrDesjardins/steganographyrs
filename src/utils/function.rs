use magic_crypt::MagicCryptError;

use super::binary::{binary_string_to_char, char_to_binary_string, pack_bit, unpack_bit};
use super::encryption::{decrypt_if_needed, encrypt_if_needed};
use super::options::{SteganographyDecryptOption, SteganographyEncryptOption};

const NUMBER_BIT_PER_BYTE: u8 = 8;

const EOF_CHAR: char = 4u8 as char;

/// From a 1d position, returns a 2d position using the width of the image
///
/// # Arguments
///
/// * `position` - The index position of the character in the string. Value varies from 0 to the length-1 of the text
/// * `width` - The width of the image to determine when to change line (height)
///
fn get_coordinate(position: u32, width: u32) -> (u32, u32) {
    let y = position / width; // No decimal
    let x = position - (width * y);

    (x, y)
}

/// Add a string (message) into an image that is referenced by a path in the `options` argument
///
/// # Arguments
///
/// * `options` - Structure with the information about the message to insert and which image to use as
///         the source and where to save the altered image that contain the secret message. The option contains
///         the detail about if the message passed in the option must be encrypted
///
pub fn add_message_to_image(options: SteganographyEncryptOption) {
    let data_to_insert = encrypt_if_needed(options.message, options.password);
    let data_to_add_with_eof = format!("{}{}", data_to_insert, EOF_CHAR);
    let data_bytes = data_to_add_with_eof.as_bytes();
    let img = image::open(options.input_image_path).unwrap();
    // let exist_img_dimension = img.dimensions();
    // println!("dimensions {:?}", exist_img_dimension);

    // Modify
    let mut new_img = img.to_rgba8();
    let mut sliding_image_position = 0; // Position into the buffer

    // The iteration order is x = 0 to width then y = 0 to height Starting from the top left.
    // Pixel are a fixed array of 4 u8
    // See: https://docs.rs/image/0.20.1/image/struct.Rgba.html
    // and see : https://doc.rust-lang.org/nightly/std/primitive.array.html

    for char_code in data_bytes {
        let char_binary = char_to_binary_string(char_code);
        let char_binary_bytes: Vec<u8> = char_binary
            .as_bytes()
            .iter()
            .map(|c| u8::from(*c != 48))
            .collect();

        let mut i = 0;
        while i < NUMBER_BIT_PER_BYTE {
            // two loops: 0 to 3 bits, 4 to 8 bits (4 because r-g-b-a)
            let coordinate = get_coordinate(sliding_image_position, img.width());
            let current_color = new_img.get_pixel(coordinate.0, coordinate.1);

            let mut new_rgba: [u8; 4] = [0, 0, 0, 0];
            for irgba in 0..4 {
                let bit = char_binary_bytes[i as usize];
                let rgba = current_color[irgba]; // RGBA
                new_rgba[irgba] = pack_bit(rgba, bit);
                i += 1;
            }
            sliding_image_position += 1;
            new_img.put_pixel(coordinate.0, coordinate.1, image::Rgba(new_rgba));
        }
    }
    // Out
    new_img.save(options.output_image_path).unwrap();
}

/// Get a string (message) from an image that is referenced by a path in the `options` argument.
/// It assumes the image was using `add_message_to_image` to find the hidden piece of information
///
/// # Arguments
///
/// * `options` - Structure with the where to find the image and detail about if the bytes retrieved
///         need to be decrypted using the password provided (optional)
///
pub fn get_message_from_image(
    options: SteganographyDecryptOption,
) -> Result<String, MagicCryptError> {
    let img = image::open(options.input_image_path).unwrap();

    let new_buffer = img.as_bytes();
    let msg = get_message_from_buffer(new_buffer);
    decrypt_if_needed(msg, options.password)
}

/// Get an array of bytes to extract the char
///
/// # Arguments
///
/// * `new_buffer` - An array of bytes that represent the whole image. Each bytes are a part of
///         the image colors.
///    The buffer has the pattern [R, G, B, A, R, G, B, A, ...]
///
pub fn get_message_from_buffer(new_buffer: &[u8]) -> String {
    let mut result = String::new();
    let mut data_position = 0;
    let mut last_character = 0 as char;
    let mut bit_counter = 0;
    let mut bits = String::new();
    while last_character != EOF_CHAR && data_position < new_buffer.len() {
        while bit_counter < NUMBER_BIT_PER_BYTE {
            let rgba_color = new_buffer[data_position];
            let bit = unpack_bit(rgba_color);
            bits.push_str(if bit == 0 { "0" } else { "1" });
            data_position += 1;
            bit_counter += 1;
        }
        last_character = binary_string_to_char(bits);
        if last_character != EOF_CHAR {
            result.push(last_character);
        }
        bit_counter = 0;
        bits = "".to_string();
    }

    result
}
#[cfg(test)]
mod test_get_string {
    use super::*;

    #[test]
    fn test_add_message_to_image() {
        let options = SteganographyEncryptOption {
            input_image_path: "testAssets/prestine.png".to_string(),
            message: "Bye".to_string(),
            output_image_path: "testAssets/out.png".to_string(),
            password: None,
        };
        add_message_to_image(options);
    }

    #[test]
    fn test_get_coordinate_first_row() {
        let result = get_coordinate(2, 10);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 0);
    }

    #[test]
    fn test_get_coordinate_second_row() {
        let result = get_coordinate(10, 10);
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 1);
    }

    #[test]
    fn test_get_coordinate_third_row() {
        let result = get_coordinate(22, 10);
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 2);
    }

    #[test]
    fn test_get_message_from_image() {
        let options = SteganographyDecryptOption {
            input_image_path: "testAssets/out_message_Bye.png".to_string(),
            password: None,
        };
        let message = get_message_from_image(options).unwrap();
        assert_eq!(message, "Bye".to_string());
    }

    #[test]
    fn test_get_message_from_buffer() {
        let buffer = [
            // Letter B
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000001".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000001".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            // End of file Character
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000001".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
            binary_string_to_char("00000000".to_string()) as u8,
        ];
        let message = get_message_from_buffer(&buffer);
        assert_eq!(message, "B".to_string());
    }
}
