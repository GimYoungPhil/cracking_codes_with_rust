#[derive(Debug)]
enum WhiskyKind {
    Blended,
    SingleMalt,
    Bourbone,
}

#[derive(Debug)]
struct Whiksy {
    kind: WhiskyKind,
    product: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let chivas_regal = Whiksy {
            kind: WhiskyKind::Blended,
            product: String::from("시바스 리갈"),
        };

        let glenfiddich = Whiksy {
            kind: WhiskyKind::SingleMalt,
            product: String::from("글렌피딕"),
        };

        let wild_turkey = Whiksy {
            kind: WhiskyKind::Bourbone,
            product: String::from("와일드 터키"),
        };

        println!("{:?}", chivas_regal);
        println!("{:?}", glenfiddich);
        println!("{:?}", wild_turkey);
    }

}
