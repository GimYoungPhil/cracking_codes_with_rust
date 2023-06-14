pub fn reverse_chiper(message: &str) -> String {
    let mut translated: Vec<u8> = Vec::with_capacity(message.len());

    for ch in message.bytes() {
        translated.push(ch);
    }

    translated.reverse();


    String::from_utf8(translated).expect("Invalid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reverse() {
        let result = reverse_chiper("Hello, world!");
        assert_eq!(result, String::from("!dlrow ,olleH"));
    }
}
