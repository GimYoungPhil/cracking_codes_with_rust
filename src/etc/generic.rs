fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = vec![34, 50, 25, 100, 65];

        assert_eq!(100, largest(&list));

        let list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        assert_eq!(6000, largest(&list));
    }
}
