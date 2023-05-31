
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn rotate_key(key: i128, length: usize) -> Option<i128> {
    match key.checked_rem(length as i128) {
        Some(v) => Some(v),
        None => None,
    }
}

pub fn rotate_key_euclid(key: i128, length: usize) -> Option<i128> {
    match key.checked_rem_euclid(length as i128) {
        Some(v) => Some(v),
        None => None,
    }
}

pub fn test_message(message: &str) {
    for ch in message.chars() {
        match SYMBOLS.find(ch) {
            Some(index) => println!("char: {}, index: {}", ch, index),
            None => println!("char: {}, index: None!!", ch),
        }
    }
}

pub fn rotate_index(key: i128, index: usize, length: usize) {

}

fn make_key(key: i32, length: usize) -> (i32, i32, i32) {
    let positive_length = length as i32;

    let encoding_key = key % positive_length;

    let decoing_key = if encoding_key > 0 {
        positive_length - encoding_key
    } else if encoding_key < 0 {
        -positive_length - encoding_key
     } else {
        0
     };

    (key, encoding_key, decoing_key)
}

struct CipherKey {
    key: i32,
    encoding_key: i32,
    decoing_key: i32,
}

fn make_key_1(key: i32, length: usize) -> CipherKey {
    let positive_length = length as i32;

    let encoding_key = match key.checked_rem(positive_length) {
        Some(v) => v,
        None => panic!(),
    };

    let decoing_key = positive_length - encoding_key;

    CipherKey {
        key,
        encoding_key,
        decoing_key
    }
}

pub enum Mode {
    Encrypt,
    Decrypt,
}

const SYMBOLS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

pub fn caesar_cipher(key: i32, mode: Mode, message: &str) -> String {
    let symbols_length = SYMBOLS.len();
    let mut translated: String = String::with_capacity(message.len());

    let mut rotated_key = key.checked_rem_euclid(symbols_length as i32).unwrap();

    for ch in message.chars() {
        // print!("ch: {}, ", ch);
        match SYMBOLS.find(ch) {
            Some(symbol_index) => {
                // print!("index: {:>2}, ", symbol_index);
                let mut translated_index: usize;

                if let Mode::Decrypt = mode {
                    rotated_key *= -1;
                }

                translated_index = symbol_index.checked_add_signed(rotated_key as isize).unwrap();
                translated_index = translated_index.checked_rem_euclid(symbols_length).unwrap();

                let s = &SYMBOLS[translated_index..translated_index+1];
                // println!("ch: {}", s);
                translated.push_str(s);
            },
            None => {
                println!();
                translated.push(ch);
            },
        }
    }

    translated
}


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
        assert_eq!(rotate_key(0, 26), Some(0));
        assert_eq!(rotate_key(1, 26), Some(1));
        assert_eq!(rotate_key(26, 26), Some(0));
        assert_eq!(rotate_key(27, 26), Some(1));
        assert_eq!(rotate_key(52, 26), Some(0));
    }

    #[test]
    fn it_make_key() {
        assert_eq!(make_key(1, 8), (1, 1, 7));
        assert_eq!(make_key(9, 8), (9, 1, 7));
        assert_eq!(make_key(17, 8), (17, 1, 7));

        assert_eq!(make_key(2, 8), (2, 2, 6));
        assert_eq!(make_key(10, 8), (10, 2, 6));
        assert_eq!(make_key(18, 8), (18, 2, 6));

        assert_eq!(make_key(3, 8), (3, 3, 5));
        assert_eq!(make_key(11, 8), (11, 3, 5));
        assert_eq!(make_key(19, 8), (19, 3, 5));

        assert_eq!(make_key(4, 8), (4, 4, 4));
        assert_eq!(make_key(12, 8), (12, 4, 4));
        assert_eq!(make_key(20, 8), (20, 4, 4));

        assert_eq!(make_key(5, 8), (5, 5, 3));
        assert_eq!(make_key(13, 8), (13, 5, 3));
        assert_eq!(make_key(21, 8), (21, 5, 3));

        assert_eq!(make_key(6, 8), (6, 6, 2));
        assert_eq!(make_key(14, 8), (14, 6, 2));
        assert_eq!(make_key(22, 8), (22, 6, 2));

        assert_eq!(make_key(7, 8), (7, 7, 1));
        assert_eq!(make_key(15, 8), (15, 7, 1));
        assert_eq!(make_key(23, 8), (23, 7, 1));

        assert_eq!(make_key(0, 8), (0, 0, 0));
        assert_eq!(make_key(8, 8), (8, 0, 0));
        assert_eq!(make_key(16, 8), (16, 0, 0));

        assert_eq!(make_key(-0, 8), (-0, 0, 0));
        assert_eq!(make_key(-8, 8), (-8, 0, 0));
        assert_eq!(make_key(-16, 8), (-16, 0, 0));

        assert_eq!(make_key(-7, 8), (-7, -7, -1));
        assert_eq!(make_key(-15, 8), (-15, -7, -1));
        assert_eq!(make_key(-23, 8), (-23, -7, -1));

        assert_eq!(make_key(-6, 8), (-6, -6, -2));
        assert_eq!(make_key(-14, 8), (-14, -6, -2));
        assert_eq!(make_key(-22, 8), (-22, -6, -2));

        assert_eq!(make_key(-5, 8), (-5, -5, -3));
        assert_eq!(make_key(-13, 8), (-13, -5, -3));
        assert_eq!(make_key(-21, 8), (-21, -5, -3));

        assert_eq!(make_key(-4, 8), (-4, -4, -4));
        assert_eq!(make_key(-12, 8), (-12, -4, -4));
        assert_eq!(make_key(-20, 8), (-20, -4, -4));

        assert_eq!(make_key(-3, 8), (-3, -3, -5));
        assert_eq!(make_key(-11, 8), (-11, -3, -5));
        assert_eq!(make_key(-19, 8), (-19, -3, -5));

        assert_eq!(make_key(-2, 8), (-2, -2, -6));
        assert_eq!(make_key(-10, 8), (-10, -2, -6));
        assert_eq!(make_key(-18, 8), (-18, -2, -6));

        assert_eq!(make_key(-1, 8), (-1, -1, -7));
        assert_eq!(make_key(-9, 8), (-9, -1, -7));
        assert_eq!(make_key(-17, 8), (-17, -1, -7));

    }
}
