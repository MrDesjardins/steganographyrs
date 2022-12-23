use image::{GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};

use super::options::{SteganographyDecryptOption, SteganographyEncryptOption};

const NUMBER_BIT_PER_BYTE: u8 = 8;

const EOF_CHAR: char = 4u8 as char;

pub fn add_message_to_image(options: SteganographyEncryptOption) {
    println!("Encrypt into an image");

    let img = image::open(options.input_image_path).unwrap();
    let exist_img_dimension = img.dimensions();
    println!("dimensions {:?}", exist_img_dimension);

    // Modify
    // let new_pixel = add_message_into_buffer(img., options);
    let new_img: RgbaImage = ImageBuffer::new(exist_img_dimension.0, exist_img_dimension.1);

    for (x, y, pixel) in new_img.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        //*pixel = image::Rgba([r, 0, b]);
    }
    // Out
    img.save(options.output_image_path).unwrap();
}

// pub fn add_message_into_buffer(
//     buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
//     dataToAdd: String,
// ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

// }

pub fn get_message_from_image(options: SteganographyDecryptOption) {
    println!("Decrypt from an image");
}
