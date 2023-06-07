// use chapter_00;
// use chapter_01;
use chapter_02::caesar::Cipher;

fn main() {
    let message = "This is my secret message.";

    let cipher = Cipher::with_key(13);

    let encoding = cipher.encrypt_message(message);
    println!("{}", encoding);

    let decoding = cipher.decrypt_message(&encoding);
    println!("{}", decoding);

    let (first, last) = (String::from("Lisa"), String::from("Su"));

    struct Name(String, String);

    let tom = Name(String::from("Jensen"), String::from("Huang"));

    const mut MY_POINT: &str = "127.0.0.1";
}
