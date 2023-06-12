pub fn get_gold(member: &str) -> &str {
    let v: Vec<&str> = member.split("::").collect();
    &v[0]
}

pub fn get_gold_and_silver(member: &str) -> (&str, &str) {
    let v: Vec<&str> = member.split("::").collect();
    (&v[0], &v[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let string = String::from("lion::tiger::leopard");
        let result = get_gold(&string);
        assert_eq!(result, "lion");
    }

    #[test]
    fn it_works_1() {
        let string = String::from("lion::tiger::leopard");
        let result = get_gold_and_silver(&string);
        assert_eq!(result, ("lion", "tiger"));
    }
}
