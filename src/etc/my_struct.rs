pub mod caesar {

    #[derive(PartialEq, Debug)]
    pub struct Cipher {
        original_key: i32,
        encoding_key: i32,
        decoding_key: i32,
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
    }
}

#[cfg(test)]
mod tests {
    use super::caesar::*;

    #[test]
    fn it_works_0() {
        let cipher = Cipher::with_key(13);
        assert_eq!(cipher.origin(), 13);
        assert_eq!(cipher.encode(), 13);
        assert_eq!(cipher.decode(), -13);
    }

    #[test]
    fn it_works_1() {
        let cipher1 = Cipher::with_key(13);
        let cipher2 = Cipher::with_key(13);
        assert_eq!(cipher1, cipher2);
    }

    #[test]
    fn it_works_2() {
        let cipher1 = Cipher::with_key(13);
        let cipher2 = Cipher::with_key(79);
        assert_eq!(cipher1.encode(), cipher2.encode());
        assert_eq!(cipher1.decode(), cipher2.decode());
    }

    #[test]
    fn it_works_3() {
        let cipher = Cipher::with_key(-13);
        assert_eq!(cipher.origin(), -13);
        assert_eq!(cipher.encode(), 53);
        assert_eq!(cipher.decode(), -53);
    }

    #[test]
    fn it_works_4() {
        let cipher1 = Cipher::with_key(-13);
        let cipher2 = Cipher::with_key(-79);
        assert_eq!(cipher1.encode(), cipher2.encode());
        assert_eq!(cipher1.decode(), cipher2.decode());
    }

}
