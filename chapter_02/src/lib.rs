pub fn add(left: usize, right: usize) -> usize {
    left + right
}

const SYMBOLS: &'static str = "ABCDEFGHIJKLMLOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

pub enum Mode {
    Encrypt,
    Decrypt,
}

pub fn rotated_key(key: usize, length: usize) -> usize {
    key % length
}

pub fn rotated_key_1(key: i128, length: usize) -> i128 {
    key % (length as i128)
}

pub fn rotated_key_2(key: i128, length: usize) -> i128 {
    key.rem_euclid(length as i128)
}

pub fn test_message(message: &str) {
    for ch in message.chars() {
        match SYMBOLS.find(ch) {
            Some(index) => println!("char: {}, index: {}", ch, index),
            None => println!("char: {}, index: None!!", ch),
        }
    }
}

/*
pub fn caesar_cipher(key: usize, mode: Mode, message: &str) -> String {
    let symbols_length = SYMBOLS.len();
    let message_length = message.len();
    let mut translated: Vec<u8> = Vec::with_capacity(message_length);

    for ch in message.chars() {
        match SYMBOLS.find(ch) {
            Some(symbol_index) => {
                let mut translated_index: usize;

                match mode {
                    Mode::Encrypt => translated_index = symbol_index + key,
                    Mode::Decrypt => translated_index = symbol_index - key,
                }

                if (translated_index >= symbols_length) {
                    translated_index -= symbols_length;
                } else {
                    translated_index += symbols_length;
                }

                match symbols.get(translated_index) {
                    Some(s) => translated.push(s),
                    None => (),
                }
            },
            None => {
                translated.push(ch);
            },
        }
    }

    String::from_utf8(translated).expect("Invalid UTF-8")
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_key_0() {
        assert_eq!(rotated_key(0, 26), 0);
        assert_eq!(rotated_key(1, 26), 1);
        assert_eq!(rotated_key(26, 26), 0);
        assert_eq!(rotated_key(27, 26), 1);
        assert_eq!(rotated_key(52, 26), 0);
    }

    #[test]
    fn it_key_1() {
        assert_eq!(rotated_key_1(0, 26), 0);
        assert_eq!(rotated_key_1(1, 26), 1);
        assert_eq!(rotated_key_1(26, 26), 0);
        assert_eq!(rotated_key_1(27, 26), 1);
        assert_eq!(rotated_key_1(52, 26), 0);

        assert_eq!(rotated_key_1(-1, 26), -1);
        assert_eq!(rotated_key_1(-26, 26), 0);
        assert_eq!(rotated_key_1(-27, 26), -1);
        assert_eq!(rotated_key_1(-52, 26), 0);
    }

    #[test]
    fn it_key_2() {
        assert_eq!(rotated_key_2(0, 26), 0);
        assert_eq!(rotated_key_2(1, 26), 1);
        assert_eq!(rotated_key_2(25, 26), 25);

        assert_eq!(rotated_key_2(26, 26), 0);
        assert_eq!(rotated_key_2(27, 26), 1);
        assert_eq!(rotated_key_2(51, 26), 25);

        assert_eq!(rotated_key_2(-26, 26), 0);
        assert_eq!(rotated_key_2(-25, 26), 1);
        assert_eq!(rotated_key_2(-1, 26), 25);

        assert_eq!(rotated_key_2(-52, 26), 0);
        assert_eq!(rotated_key_2(-51, 26), 1);
        assert_eq!(rotated_key_2(-27, 26), 25);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(7_i64.div_euclid(4), 1);
        assert_eq!(7_i64.div_euclid(-4), -1);
        assert_eq!((-7_i64).div_euclid(4), -2);
        assert_eq!((-7_i64).div_euclid(-4), 2);
        
        // assert_eq!(7_i64.div_euclid(0), -1);
        // assert_eq!((-7_i64).div_euclid(0), 2);

        assert_eq!(7_i64.rem_euclid(4), 3);
        assert_eq!(7_i64.rem_euclid(-4), 3);
        assert_eq!((-7_i64).rem_euclid(4), 1);
        assert_eq!((-7_i64).rem_euclid(-4), 1);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(7_i64.div_euclid(1), 7);
        assert_eq!(7_i64.rem_euclid(1), 0);

        assert_eq!(7_i64.div_euclid(-1), -7);
        assert_eq!(7_i64.rem_euclid(-1), 0);

        assert_eq!((-7_i64).div_euclid(1), -7);
        assert_eq!((-7_i64).rem_euclid(1), 0);

        assert_eq!((-7_i64).div_euclid(-1), 7);
        assert_eq!((-7_i64).rem_euclid(-1), 0);
    }

    #[test]
    fn it_works_3() {
        assert_eq!(127_i8.div_euclid(1), 127);
        assert_eq!(127_i8.rem_euclid(1), 0);

        assert_eq!(127_i8.div_euclid(-1), -127);
        assert_eq!(127_i8.rem_euclid(-1), 0);

        assert_eq!((-128_i8).div_euclid(1), -128);
        assert_eq!((-128_i8).rem_euclid(1), 0);

        // assert_eq!((-128_i8).div_euclid(-1), 128);
        // assert_eq!((-128_i8).rem_euclid(-1), 0);
    }
}
