
pub struct MyNumber {
    pub numbers: Vec<i32>,
}

impl Iterator for MyNumber {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.numbers.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn works_13_0() {
        let my = MyNumber {
            numbers: vec![2, 4, 6, 8, 10],
        };

        // let my_iter = my.iter();
    }

    #[test]
    fn works_13_2() {

        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        println!("{:?}", v1);
    }

    #[test]
    fn works_13_3() {

        let mut v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter_mut();

        assert_eq!(v1_iter.next(), Some(&mut 1));
        assert_eq!(v1_iter.next(), Some(&mut 2));
        assert_eq!(v1_iter.next(), Some(&mut 3));
        assert_eq!(v1_iter.next(), None);

        println!("{:?}", v1);
    }

    #[test]
    fn works_13_4() {

        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.into_iter();

        assert_eq!(v1_iter.next(), Some(1));
        assert_eq!(v1_iter.next(), Some(2));
        assert_eq!(v1_iter.next(), Some(3));
        assert_eq!(v1_iter.next(), None);

        // println!("{:?}", v1);
    }

}
