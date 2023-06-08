pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod caesar {

    pub struct Cipher {
        original_key: i32,
        encoding_key: i32,
        decoding_key: i32,
    }

    enum Mode {
        Encrypt,
        Decrypt,
    }

    const SYMBOLS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

    impl Cipher {

        pub fn with_key(key: i32) -> Cipher {
            let symbols_length = SYMBOLS.len() as i32;
            let encoding_key = key.rem_euclid(symbols_length);
            Cipher {
                original_key: key,
                encoding_key,
                decoding_key: -encoding_key,
            }
        }

        pub fn origin(&self) -> i32 {
            self.original_key
        }

        pub fn encode(&self) -> i32 {
            self.encoding_key
        }

        pub fn decode(&self) -> i32 {
            self.decoding_key
        }

        pub fn encrypt_message(&self, message: &str) -> String {
            self.translate_message(message, Mode::Encrypt)
        }

        pub fn decrypt_message(&self, message: &str) -> String {
            self.translate_message(message, Mode::Decrypt)
        }

        fn translate_message(&self, message: &str, mode: Mode) -> String {

            // let symbols_length = SYMBOLS.len() as i32;

            // let cipher_key = self.key % symbols_length;

            let mut translated = String::with_capacity(message.len());

            for ch in message.chars() {
                
                match SYMBOLS.find(ch) {
                    Some(symbol_index) => {

                        let moved_index = symbol_index as i32 + match mode {
                            Mode::Encrypt => self.encode(),
                            Mode::Decrypt => self.decode(),
                        };

                        let translated_index: usize = moved_index.rem_euclid(10) as usize;
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
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cipher = caesar::Cipher::with_key(13);
        assert_eq!(cipher.origin(), 13);
        assert_eq!(cipher.encode(), 13);
        assert_eq!(cipher.decode(), -13);
    }
}
