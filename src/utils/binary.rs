/// Reads the least significant bits of the pixel (Red, Green, Blue, Alpha) and
/// adds them to the corresponding position of the byte being constructed
///
/// # Arguments
///
///  * buffer_item - 8 bits number (E.g. 0b000011u8)
///
/// # Returns
///
/// Return 0 or 1 depending of the last bit of the buffer_item
///
pub fn unpack_bit(buffer_item: u8) -> u8 {
    // If the bufferItem (R or G or B or A) finish with a 1 (instead of of 0)
    let last_digit = buffer_item & (1 << 0);
    u8::from(last_digit != 0)
}

/// Sets the least significant bit to 1 or 0 (depending on the bit to set)
///
/// # Arguments
///  * buffer_item - 8 bits number (E.g. 0b000011u8)
///  * bit - 0 or 1
///
/// # Returns
/// Modified buffer_item with the last bit set to the bit passed by parameter
pub fn pack_bit(buffer_item: u8, bit: u8) -> u8 {
    if bit == 0 {
        return buffer_item & !(1 << 0);
    }
    // Else 1
    buffer_item | (1 << 0)
}

/// Takes a char like 65 and convert it to "1001111";
///
/// # Arguments
///  * char_code - The ASCII value as a number (0 to 255)
///
/// # Returns
/// Always a 8 characters string (8 bits)
pub fn char_to_binary_string(char_code: &u8) -> String {
    format!("{:08b}", char_code)
}

/// Extract a binary ("10011001") to a char
///
/// # Arguments
///  * input - A string of 8 characters shaped with only 1 and 0 characters
///
/// # Returns
/// A character from the value of the input
pub fn binary_string_to_char(input: String) -> char {
    // let num = i8::from_str_radix(&input, 2).unwrap() as u8;
    let num = u8::from_str_radix(&input, 2);
    match num {
        Ok(n) => n as char,
        Err(err) => {
            eprintln!("{:?}", err);
            '0'
        }
    }
}

#[cfg(test)]
mod test_get_string {
    use super::*;
    
    #[test]
    fn test_char_to_binary_string_code_65() {
        let result = char_to_binary_string(&65);
        assert_eq!(result, "01000001")
    }
    #[test]
    fn test_char_to_binary_string_code_2() {
        let result = char_to_binary_string(&2);
        assert_eq!(result, "00000010")
    }

    #[test]
    fn test_binary_string_to_char_a() {
        let result = binary_string_to_char("01000001".to_string());
        assert_eq!(result, 'A')
    }
    #[test]
    fn test_binary_string_to_char_eth() {
        let result = binary_string_to_char("11010001".to_string());
        assert_eq!(result, 'Ñ')
    }
    #[test]
    fn test_binary_string_to_char_overflow() {
        let result = binary_string_to_char("101000001".to_string());
        assert_eq!(result, '0')
    }
    #[test]
    fn test_str_radix() {
        let result = u8::from_str_radix("11010001", 2).unwrap();
        assert_eq!(result, 209);
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
