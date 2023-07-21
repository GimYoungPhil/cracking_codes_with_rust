fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
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
    fn works_1() {
        let list = [10, 20, 30, 50, 70, 100, 90];

        assert_eq!(largest_i32(&list), &100);
    }

    #[test]
    fn works_2() {
        let list = ['a', 's', 'c', 'e', 'f', 'g', 'h'];

        assert_eq!(largest_char(&list), &'s');
    }

    #[test]
    fn works_3() {

        let list = [10, 20, 30, 50, 70, 100, 90];

        assert_eq!(largest(&list), &100);

        let list = ['a', 's', 'c', 'e', 'f', 'g', 'h'];

        assert_eq!(largest(&list), &'s');
    }
}
