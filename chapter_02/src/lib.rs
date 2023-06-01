pub enum Mode {
    Encrypt,
    Decrypt,
}

const SYMBOLS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

pub fn caesar_cipher(key: i32, mode: Mode, message: &str) -> String {

    let symbols_length = SYMBOLS.len() as i32;

    let cipher_key = key % symbols_length;

    let mut translated: String = String::with_capacity(message.len());

    for ch in message.chars() {
        // print!("ch: {}, ", ch);
        match SYMBOLS.find(ch) {
            Some(symbol_index) => {
                // print!("in: {:>2}, ", symbol_index);
                let moved_index = match mode {
                    Mode::Encrypt => symbol_index as i32 + cipher_key,
                    Mode::Decrypt => symbol_index as i32 - cipher_key,
                };
                // print!("mo: {:>2}, ", moved_index);
                let translated_index = moved_index.rem_euclid(symbols_length) as usize;
                // print!("ou: {:>2}, ", translated_index);
                translated.push_str(&SYMBOLS[translated_index..translated_index+1]);
            },
            None => {
                // println!();
                translated.push(ch);
            },
        }
        println!();
    }

    translated
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_make_key() {

    }
}
