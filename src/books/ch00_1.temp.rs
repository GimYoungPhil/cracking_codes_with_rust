fn ex_01() -> i32 {
    1
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        assert_eq!(ex_01(), 1);
    }
}
