// use chapter_00;
// use chapter_01;
use chapter_02::{self, Mode};

fn main() {
    let message = "This is my secret message.";
    // println!("{}", chapter_01::reverse_chiper(message));
    // chapter_02::test_message(message);
    let encode = chapter_02::caesar_cipher(13, Mode::Encrypt, message);
    println!("{}", encode);
    let decode = chapter_02::caesar_cipher(13, Mode::Decrypt, &encode);
    println!("{}", decode);

    chapter_02::print_test();
}
