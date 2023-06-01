// use chapter_00;
// use chapter_01;
use chapter_02::{self, Mode};

fn main() {
    let key = 13;
    let message = "This is my secret message.";

    let encode = chapter_02::caesar_cipher(key, Mode::Encrypt, message);
    println!("{}", encode);

    let decode = chapter_02::caesar_cipher(key, Mode::Decrypt, &encode);
    println!("{}", decode);
}
