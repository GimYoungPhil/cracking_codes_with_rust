use std::ops::Deref;
use List::{Cons, Nil};

pub fn run() {
    let b = Box::new(5);
    println!("b = {b}");
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn run_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let b = Box::new(5);
        let p = &b;

        println!("{}", b);
        println!("{}", p);
        println!("{}", &b);
        println!("{}", &p);

        println!("{:p}", b);
        println!("{:p}", p);
        println!("{:p}", &b);
        println!("{:p}", &p);

        assert_eq!(&20, &100);
    }

    #[test]
    fn it_works_2() {
        let x: i32 = 5;
        let y: &i32 = &x;

        println!("{}", x);
        println!("{}", y);
        println!("{}", &x);
        println!("{}", &y);

        // println!("{:p}", a);
        // println!("{:p}", y);
        // println!("{:p}", &x);
        // println!("{:p}", &y);

        assert_eq!(x, 5);
        // assert_eq!(y, 5);

        assert_eq!(*y, 5);
        assert_eq!(y, &5);
    }

    #[test]
    fn it_works_3() {
        let x: i32 = 5;
        let y: Box<i32> = Box::new(x);

        assert_eq!(x, 5);
        assert_eq!(*y, 5);
    }

    #[test]
    fn it_works_4() {
        let x: i32 = 5;
        let y: MyBox<i32> = MyBox::new(x);

        assert_eq!(x, 5);
        assert_eq!(*y, 5);
    }

    #[test]
    fn it_works_5() {
        let m: MyBox<String> = MyBox::new(String::from("Rust"));
        let k: &MyBox<String> = &m;
        let s: &String = k;
        let t: &str = s;

        assert_eq!(t, "Rust");
        assert_eq!(&(*m)[..], "Rust");
    }
}
