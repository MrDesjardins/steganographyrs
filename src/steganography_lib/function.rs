use image::{GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};

use crate::steganography_lib::binary::{char_to_binary_string, pack_bit};

use super::options::{SteganographyDecryptOption, SteganographyEncryptOption};

const NUMBER_BIT_PER_BYTE: u8 = 8;

const EOF_CHAR: char = 4u8 as char;

pub fn add_message_to_image(options: SteganographyEncryptOption) {
    println!("Encrypt into an image");
    let data_to_add_with_eof = format!("{}{}", options.message, EOF_CHAR);
    let data_bytes = data_to_add_with_eof.as_bytes();
    let img = image::open(options.input_image_path).unwrap();
    let exist_img_dimension = img.dimensions();
    println!("dimensions {:?}", exist_img_dimension);

    // Modify
    // let new_pixel = add_message_into_buffer(img., options);
    //let mut new_img: RgbaImage = ImageBuffer::new(exist_img_dimension.0, exist_img_dimension.1);
    let mut new_img = img.to_rgba8();
    let mut sliding_image_position = 0; // Position into the buffer

    // The iteration order is x = 0 to width then y = 0 to height Starting from the top left.
    // Pixel are a fixed array of 4 u8
    // See: https://docs.rs/image/0.20.1/image/struct.Rgba.html
    // and see : https://doc.rust-lang.org/nightly/std/primitive.array.html

    for data_position in 0..data_bytes.len() {
        let char_code = data_bytes[data_position];
        let char_binary = char_to_binary_string(char_code);
        let char_binary_bytes = char_binary.as_bytes();

        for i in 0..NUMBER_BIT_PER_BYTE {
            let bit = char_binary_bytes[i as usize];
            for _j in 0..2 {
                // two loops: 0 to 3 bits, 4 to 8 bits (4 because r-g-b-a)
                let coordinate = get_coordinate(sliding_image_position, img.width());
                let current_color = new_img.get_pixel(coordinate.0, coordinate.1);

                let mut new_rgba: [u8; 4] = [0, 0, 0, 0];
                for irgba in 0..4 {
                    let rgba = current_color[irgba]; // RGBA
                    new_rgba[irgba] = pack_bit(rgba, bit);
                }
                sliding_image_position += 1;
                new_img.put_pixel(coordinate.0, coordinate.1, image::Rgba(new_rgba));
            }
        }
    }
    // Out
    img.save(options.output_image_path).unwrap();
}

// pub fn add_message_into_buffer(
//     buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
//     dataToAdd: String,
// ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

// }

fn get_coordinate(position: u32, width: u32) -> (u32, u32) {
    let y = position / width; // No decimal
    let x = position - (width * y);

    (x, y)
}

pub fn get_message_from_image(options: SteganographyDecryptOption) {
    println!("Decrypt from an image");
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
            password: "123".to_string(),
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
}
