use super::constants::NUMBER_BIT_PER_BYTE;

/*
Reads the least significant bits of the pixel (Red, Green and Blue) and
add them to the corresponding position of the byte being constructed
*/
pub fn unpack_bit(buffer_item: u8) -> u8 {
    // If the bufferItem (R or G or B or A) finish with a 1 (instead of of 0)
    let last_digit = buffer_item & (1 << 0);
    return if last_digit == 0 { 0 } else { 1 };
}

/*
Sets the least significant bit to 1 or 0 (depending on the bit to set)
*/
pub fn pack_bit(buffer_item: u8, bit: u8) -> u8 {
    if bit == 0 {
        return buffer_item & !(1 << 0);
    }
    // Else 1
    return buffer_item | (1 << 0);
}

/**
 * Takes a char like 65 and convert it to "1001111";
 * Always return 8 bits
 **/
pub fn char_to_binary_string(char_code: u8) -> String {
    format!("{:08b}", char_code)
}

/**
 * Extract a binary ("10011001") to a char
 **/
pub fn binary_string_to_char(input: String) -> char {
    let num = i8::from_str_radix(&input, 2).unwrap() as u8;
    num as char
}

/**
 * Evaluate the number of byte required for an image to embedded the desired message.
 *
 * Can be used to avoid inserting message on a small image or to choose in a collection
 * of image the best one to embedded the message.
 *
 * Assume the message as the EOF character
 **/
pub fn get_minimum_image_size_byte_required(message: String) -> usize {
    let total_letter = message.chars().count();
    total_letter * NUMBER_BIT_PER_BYTE as usize
}

#[cfg(test)]
mod test_get_string {
    use super::*;

    #[test]
    fn test_get_minimum_image_size_byte_required_three_chars() {
        let result = get_minimum_image_size_byte_required("Bye".to_string());
        assert_eq!(result, 24)
    }

    #[test]
    fn test_char_to_binary_string_code_65() {
        let result = char_to_binary_string(65);
        assert_eq!(result, "01000001")
    }

    #[test]
    fn test_binary_string_to_char_a() {
        let result = binary_string_to_char("01000001".to_string());
        assert_eq!(result, 'A')
    }
    #[test]
    fn test_char_to_binary_string_code_2() {
        let result = char_to_binary_string(2);
        assert_eq!(result, "00000010")
    }
    #[test]
    fn test_unpack_number_finish_with_one() {
        let result = unpack_bit(0b000011u8);
        assert_eq!(result, 1)
    }
    #[test]
    fn test_unpack_number_finish_with_zero() {
        let result = unpack_bit(0b000010u8);
        assert_eq!(result, 0)
    }
    #[test]
    fn test_pack_number_finish_with_zero_set_one() {
        let result = pack_bit(90, 1);
        assert_eq!(result, 91)
    }
    #[test]
    fn test_pack_number_finish_with_one_set_one() {
        let result = pack_bit(91, 1);
        assert_eq!(result, 91)
    }
    #[test]
    fn test_pack_number_finish_with_zero_set_zero() {
        let result = pack_bit(90, 0);
        assert_eq!(result, 90)
    }
    #[test]
    fn test_pack_number_finish_with_one_set_zero() {
        let result = pack_bit(91, 0);
        assert_eq!(result, 90)
    }
}
