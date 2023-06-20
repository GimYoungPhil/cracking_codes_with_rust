fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    println!("{}", list.len());
    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = vec![1, 2, 3, 4, 5, 34, 50, 25, 100, 65];
        println!("{}", list.len());

        let result = largest(&list);
        println!("{}", result);

        assert_eq!(&100, result);

        // let list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        // assert_eq!(&6000, largest(&list));
    }
}
