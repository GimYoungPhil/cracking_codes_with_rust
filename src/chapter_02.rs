pub mod caesar {

    pub struct Cipher {
        key: i32,
    }

    enum Mode {
        Encrypt,
        Decrypt,
    }

    const SYMBOLS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

    impl Cipher {

        pub fn with_key(key: i32) -> Cipher {
            Cipher { key }
        }

        fn translate_message(&self, message: &str, mode: Mode) -> String {

            let symbols_length = SYMBOLS.len() as i32;

            let cipher_key = self.key % symbols_length;

            let mut translated = String::with_capacity(message.len());

            for ch in message.chars() {

                match SYMBOLS.find(ch) {
                    Some(symbol_index) => {

                        let moved_index = match mode {
                            Mode::Encrypt => symbol_index as i32 + cipher_key,
                            Mode::Decrypt => symbol_index as i32 - cipher_key,
                        };

                        let translated_index: usize = moved_index.rem_euclid(symbols_length) as usize;
                        translated.push_str(&SYMBOLS[translated_index..translated_index+1]);

                        println!("ch: {:>2}, in: {:>2}, mo: {:>2}, ou: {:>2}", ch, symbol_index, moved_index, translated_index);
                    },
                    None => {
                        translated.push(ch);

                        println!("ch: {:>2}, No match", ch);
                    },
                }
            }

            translated
        }

        pub fn encrypt_message(&self, message: &str) -> String {
            self.translate_message(message, Mode::Encrypt)
        }

        pub fn decrypt_message(&self, message: &str) -> String {
            self.translate_message(message, Mode::Decrypt)
        }


    }

}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_make_key() {

    }
}
