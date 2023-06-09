pub mod caesar {
    pub struct Cipher {
        pub original_key: i32,
        pub encoding_key: i32,
        pub decoding_key: i32,
    }

    enum Mode {
        Encrypt,
        Decrypt,
    }

    impl Cipher {
        const SYMBOLS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";
        const SYMBOLS_LENGTH: i32 = 66;

        pub fn with_key(key: i32) -> Result<Cipher, &'static str> {
            let encoding_key = match key.checked_rem_euclid(Cipher::SYMBOLS_LENGTH) {
                Some(v) => v,
                None => {
                    return Err("");
                },
            };

            Ok(Cipher {
                original_key: key,
                encoding_key: encoding_key,
                decoding_key: -encoding_key,
            })
        }

        // pub fn encrypt_message(&self, message: &str) -> String {
        //     self.translate_message(message, Mode::Encrypt)
        // }

        // pub fn decrypt_message(&self, message: &str) -> String {
        //     self.translate_message(message, Mode::Decrypt)
        // }

        pub fn translate_message(&self, message: &str, mode: &str) -> Result<String, &'static str> {

            let mut translated = String::with_capacity(message.len());

            for ch in message.chars() {
                
                match Cipher::SYMBOLS.find(ch) {
                    Some(symbol_index) => {

                        // let moved_index = symbol_index as i32 + match mode {
                        //     Mode::Encrypt => self.encoding_key,
                        //     Mode::Decrypt => self.decoding_key,
                        // };

                        let moved_index = symbol_index as i32 + if mode == "encoding" {
                            self.encoding_key
                        } else if mode == "decoding" {
                            self.decoding_key
                        } else {
                            return Err("not enough arguments");
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

            Ok(translated)
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
