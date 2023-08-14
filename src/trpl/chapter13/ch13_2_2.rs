#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}


fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let v = [1, 2, 3, 4, 5];
        let v_iter = v.iter();

        let total: i32 = v_iter.sum();

        assert_eq!(total, 15);
    }

    #[test]
    fn works_2() {
        let v = [1, 2, 3, 4, 5];
        let v_iter = v.iter();

        let mut other = v_iter.map(|x| x + 10);

        assert_eq!(other.next(), Some(11));
        assert_eq!(other.next(), Some(12));
        assert_eq!(other.next(), Some(13));
        assert_eq!(other.next(), Some(14));
        assert_eq!(other.next(), Some(15));
        assert_eq!(other.next(), None);
    }

    #[test]
    fn works_3() {
        let v = [1, 2, 3, 4, 5];
        let v_iter = v.iter();

        let my_collect: Vec<_> = v_iter.map(|x| x * 10).collect();

        println!("{:?}", my_collect);
    }

    
    #[test]
    fn works_4() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 16,
                style: String::from("boot"),
            },
            Shoe {
                size: 9,
                style: String::from("ship"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
            ]
        );
    }
}
