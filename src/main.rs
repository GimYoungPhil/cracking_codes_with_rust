// use cracking_codes_with_rust::chapters::chapter_00;
use cracking_codes_with_rust::chapters::chapter_02::caesar;

// use chapter_00;
// use chapter_01;
// use chapter_02::caesar::Cipher;

fn main() {

    // chapter_00::some_0();
    // chapter_00::some_1();

    let message = "This is my secret message.";
    let cipher = caesar::Cipher::with_key(13);

    let encoding = cipher.encrypt_message(message);
    println!("{}", encoding);

    let decoding = cipher.decrypt_message(&encoding);
    println!("{}", decoding);

}
