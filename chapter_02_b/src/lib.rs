pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn test_rem(x: i32, y: i32) -> i32 {
    match x.checked_rem_euclid(y) {
        Some(x) => x,
        None => panic!("if x is MIN, and y is {}, it'll be panied", y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        assert_eq!(test_rem(13, 10), 3);
    }

    #[test]
    #[should_panic(expected = "if x is MIN, and y is -1, it'll be panied")]
    fn it_works_1() {
        test_rem(i32::MIN, -1);
    }
}
