pub fn get_len(member: &str) -> usize {
    member.len()
}

pub fn get_gold<'a>(member: &'a str) -> &'a str {
    let v: Vec<&str> = member.split("::").collect();
    &v[0]
}

pub fn get_gold_and_silver<'a>(member: &'a str) -> (&'a str, &'a str) {
    let v: Vec<&str> = member.split("::").collect();
    (&v[0], &v[1])
}

pub fn get_first<'a>(member: &'a str, ss: &'a str) -> &'a str {
    let v: Vec<&str> = member.split(ss).collect();
    &v[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0 () {
        let length;
        {
            let string = String::from("tiger");
            length = get_len(&string);
        }

        assert_eq!(length, 5);
    }

    #[test]
    fn it_works_1() {
        let string = String::from("lion::tiger::leopard");
        let result = get_gold(&string);
        assert_eq!(result, "lion");
    }

    #[test]
    fn it_works_2() {
        let string = String::from("lion::tiger::leopard");
        let result = get_gold_and_silver(&string);
        assert_eq!(result, ("lion", "tiger"));
    }

    #[test]
    fn it_works_3() {
        let string = String::from("lion::tiger::leopard");
        let ss: String = String::from("::");
        let result = get_first(&string, &ss);
        assert_eq!(result, "lion");
    }
}
