pub enum Mode {
    Encrypt,
    Decrypt,
}

pub fn caesar_cipher(key: i32, mode: Mode, message: &str) -> String {

    let symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

    let symbols_length = symbols.len() as i32;

    let cipher_key = key % symbols_length;

    let mut translated: String = String::with_capacity(message.len());

    for ch in message.chars() {
        match symbols.find(ch) {
            Some(symbol_index) => {
                let moved_index = match mode {
                    Mode::Encrypt => symbol_index as i32 + cipher_key,
                    Mode::Decrypt => symbol_index as i32 - cipher_key,
                };

                let translated_index = (moved_index % symbols_length) as usize;

                translated.push_str(&symbols[translated_index..translated_index+1]);
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
    fn it_make_key() {

    }
}
