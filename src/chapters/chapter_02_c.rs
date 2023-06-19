pub mod caesar {

    enum CipherType {
        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
    }

    pub struct Cipher<T> {
        pub original_key: T,
        pub encoding_key: T,
        pub decoding_key: T,
    }

    enum Mode {
        Encrypt,
        Decrypt,
    }

    impl<T> Cipher<T> {
        const SYMBOLS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";
        const SYMBOLS_LENGTH: T = 66;

        pub fn with_key(key: T) -> Cipher<T> {
            let encoding_key = key.rem_euclid(Cipher::SYMBOLS_LENGTH);
            Cipher {
                original_key: key,
                encoding_key,
                decoding_key: -encoding_key,
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
                
                match Cipher::SYMBOLS.find(ch) {
                    Some(symbol_index) => {

                        let moved_index = symbol_index as i32 + match mode {
                            Mode::Encrypt => self.encoding_key,
                            Mode::Decrypt => self.decoding_key,
                        };

                        let translated_index: usize = moved_index.rem_euclid(Cipher::SYMBOLS_LENGTH) as usize;
                        let translated_ch= &Cipher::SYMBOLS[translated_index..translated_index+1];
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
