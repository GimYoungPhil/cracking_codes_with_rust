// use chapter_00;
// use chapter_01;
use chapter_02::{self, Mode};

fn main() {
    let message = "There can keep a secret, if two of them are dead.";
    // println!("{}", chapter_01::reverse_chiper(message));
    // chapter_02::test_message(message);
    chapter_02::caesar_cipher(-144, Mode::Encrypt, message);
}
