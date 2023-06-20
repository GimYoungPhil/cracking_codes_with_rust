fn example() {
    let list = vec![1, 2, 3, 4, 5];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("{:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn example2() {
    let mut list = vec![1, 2, 3, 4, 5];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(10);

    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        example();
    }

    #[test]
    fn it_works2() {
        example2();
    }
}
