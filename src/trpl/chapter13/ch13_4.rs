#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_13_7() {
        let mut list = [
            Reactangle { width: 10, height: 1 },
            Reactangle { width: 3, height: 5 },
            Reactangle { width: 7, height: 12 },
        ];
    
        list.sort_by_key(|r| r.width);
        println!("{:?}", list);
    }

    // #[test]
    // fn works_13_8() {
    //     let mut list = [
    //         Reactangle { width: 10, height: 1 },
    //         Reactangle { width: 3, height: 5 },
    //         Reactangle { width: 7, height: 12 },
    //     ];

    //     let mut sort_operations = vec![];
    //     let value = String::from("by key called");

    //     list.sort_by_key(|r| {
    //         sort_operations.push(value);
    //         r.width
    //     });
    
    //     println!("{:?}", list);
    // }

    #[test]
    fn works_13_9() {
        let mut list = [
            Reactangle { width: 10, height: 1 },
            Reactangle { width: 3, height: 5 },
            Reactangle { width: 7, height: 12 },
        ];

        let mut num_sort_operations = 0;

        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
    
        println!("{:?}", list);
    }
}
