fn borrow_i32(v: i32) {
    println!("borrow value: {v}");
}

fn borrow_mut_i32(mut v: i32) {
    v = v + 10;
    println!("borrow value: {v}");
}

fn immutable_i32(v: &i32) {
    println!("immutable value: {v}");
}

fn mutable_i32(v: &mut i32) {
    *v = *v + 10;
    println!("mutable value: {v}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        let a = 10;
        let b = &a;

        println!("a: {}, *b: {}", a, *b);
        assert_eq!(a, *b);

        println!("&a: {:p}, b: {:p}", &a, b);
        assert_eq!(&a, b);
    }

    #[test]
    fn it_works_1() {
        let mut a = 10;
        let b = &mut a;
        *b = 20;
        println!("b: {}", b);
    }

    #[test]
    fn it_works_2() {
        let a = 10;
        borrow_i32(a);
        println!("a: {}", a);
    }

    #[test]
    fn it_works_3() {
        let a = 10;
        borrow_mut_i32(a);
        println!("a: {}", a);
    }

    #[test]
    fn it_works_4() {
        let a = 10;
        immutable_i32(&a);
        println!("a: {}", a);
    }

    #[test]
    fn it_works_5() {
        let mut a = 10;
        mutable_i32(&mut a);
        println!("a: {}", a);
    }
}
