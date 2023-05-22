pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub enum Mode {
    Encrypt,
    Decrypt,
}

pub fn caesar_cipher(key: usize, mode: Mode, message: &str) -> String {

    let symbols = "ABCDEFGHIJKLMLOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";
    let symbols_length = symbols.len();
    let mut translated: Vec<u8> = Vec::with_capacity(message.len());

    for ch in message.bytes() {
        match symbols.find(ch as char) {
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

    String::from("result")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
