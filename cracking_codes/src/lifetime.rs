fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let string1 = String::from("abcde");
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        assert_eq!(result, "abcde");
    }

    #[test]
    fn it_works_1() {
        let string1 = String::from("abcde");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            assert_eq!(result, "abcde");
        }
    }

    #[test]
    fn it_works_2() {
        let string1 = String::from("abcde");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        assert_eq!(result, "abcde");
    }

    #[test]
    fn it_works_3() {
        let string1 = String::from("abcde");
        let string2;
        let result;
        {
            string2 = String::from("xyz");
        }
        result = longest(string1.as_str(), string2.as_str());
        assert_eq!(result, "abcde");
    }

}
