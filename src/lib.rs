use std::error::Error;
use std::fs;

pub struct Config {
    pub mode: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let mode = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { mode, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())

    // let message = "This is my secret message.";
    // let cipher = caesar::Cipher::with_key(13);

    // let encoding = cipher.encrypt_message(message);
    // println!("{}", encoding);

    // let decoding = cipher.decrypt_message(&encoding);
    // println!("{}", decoding);
}

pub mod chapters {
    // pub mod chapter_00;
    pub mod chapter_02;
    // pub mod chapter_02_c;
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::chapters::chapter_02::caesar::Cipher;

    #[test]
    fn one_result() {
        let key = 13;
        let mode = "encoding";
        let contents = "\
This is my secret message.";

        assert_eq!(
            String::from("sdf"),
            Cipher::with_key(13).translate_message(contents, mode).unwrap()
        );
    }
}
