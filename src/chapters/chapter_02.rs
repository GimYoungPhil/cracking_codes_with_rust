pub mod caesar {

    pub struct Cipher {
        pub original_key: i32,
        pub encoding_key: i32,
        pub decoding_key: i32,
        pub symbols_length: i32,
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
                symbols_length,
            }
        }

        pub fn encrypt_message(&self, message: &str) -> String {
            self.translate_message(message, Mode::Encrypt)
        }

        pub fn decrypt_message(&self, message: &str) -> String {
            self.translate_message(message, Mode::Decrypt)
        }

        fn translate_message(&self, message: &str, mode: Mode) -> String {

            let mut translated = String::with_capacity(message.len());

            for ch in message.chars() {
                
                match SYMBOLS.find(ch) {
                    Some(symbol_index) => {

                        let moved_index = symbol_index as i32 + match mode {
                            Mode::Encrypt => self.encoding_key,
                            Mode::Decrypt => self.decoding_key,
                        };

                        let translated_index: usize = moved_index.rem_euclid(self.symbols_length) as usize;
                        let translated_ch= &SYMBOLS[translated_index..translated_index+1];
                        translated.push_str(translated_ch);

                        println!("ch: {:>2}, in: {:>2}, mo: {:>2}, ou: {:>2}, to: {:>2}", ch, symbol_index, moved_index, translated_index, translated_ch);
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
    fn it_works_0() {
        let cipher = caesar::Cipher::with_key(13);
        assert_eq!(cipher.original_key, 13);
        assert_eq!(cipher.encoding_key, 13);
        assert_eq!(cipher.decoding_key, -13);
    }

    #[test]
    fn it_works_1() {
        let cipher = caesar::Cipher::with_key(-13);
        assert_eq!(cipher.original_key, -13);
        assert_eq!(cipher.encoding_key, 53);
        assert_eq!(cipher.decoding_key, -53);
    }

    #[test]
    fn it_works_2() {
        let cipher = caesar::Cipher::with_key(79);
        assert_eq!(cipher.original_key, 79);
        assert_eq!(cipher.encoding_key, 13);
        assert_eq!(cipher.decoding_key, -13);
    }

    #[test]
    fn it_works_3() {
        let cipher = caesar::Cipher::with_key(-79);
        assert_eq!(cipher.original_key, -79);
        assert_eq!(cipher.encoding_key, 53);
        assert_eq!(cipher.decoding_key, -53);
    }
}