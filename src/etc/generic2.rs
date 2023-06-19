fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
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

        let list = vec!['A', 'b', 'c', 'D', 'E', 'Z', 'z', '1'];

        assert_eq!('z', largest(&list));
    }

    #[test]
    fn it_works_0() {
        let x = 10;
        let y = &x;

        assert_eq!(x, 10);
        assert_eq!(y, &10);
        assert_eq!(*y, 10);

        let a = 10;
        let b = &a;

        assert_eq!(y, b);
    }
}
