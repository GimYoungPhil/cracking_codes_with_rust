fn ex1() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();

    for i in v_iter {
        println!("value: {}", i);
    }
}

fn ex2() {
    let mut v = vec![1, 2, 3];
    let v_iter = v.iter_mut();

    for i in v_iter {
        *i += 100;
    }

    println!("{:?}", v);
}

fn ex3() {
    let v = vec![1, 2, 3, 4, 5];
    let v_iter = v.into_iter();

    let mut sum = 0;
    for i in v_iter {
        sum += i;
    }

    println!("{}", sum);
}

fn ex4() {
    let v = [10, 20, 30];
    let v_iter = v.iter();

    for i in v_iter {
        println!("value: {}", i);
    }
}

fn ex5() {
    let mut v = [10, 20, 30];
    let v_iter = v.iter_mut();

    for i in v_iter {
        *i += 5;
    }
    println!("{:?}", v);
}

fn ex6() {
    let v = [10, 20, 30];
    let v_iter = v.into_iter();

    for i in v_iter {
        println!("value: {}", i);
    }
    println!("{:?}", v);
}


fn ex7() {
    let v = &[10, 20, 30];
    let v_iter = v.iter();

    for i in v_iter {
        println!("value: {}", i);
    }
}

fn ex8() {
    let v = &mut [10, 20, 30];
    let v_iter = v.iter_mut();

    for i in v_iter {
        println!("value: {}", i);
    }
}

fn ex9() {
    let v = [10, 20, 30];
    let v_iter = v.into_iter();

    for i in v_iter {
        println!("value: {}", i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {

        ex4();
    }

    #[test]
    fn works_0() {
        let list: [i32; 3] = [100, 200, 300];
        let mut list: [i32; 3] = [100, 200, 300];
        let list: &[i32; 3] = &[100, 200, 300];
        let list: &mut [i32; 3] = &mut [100, 200, 300];

        let v = vec![1, 2, 3];
        let mut v = vec![1, 2, 3];
        let v = &vec![1, 2, 3];
        let v = &mut vec![1, 2, 3];
    }

    #[test]
    fn works_1() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        println!("{:?}", v1);
    }

    #[test]
    fn works_2() {
        let mut v2 = vec![1, 2, 3];
        let mut v2_iter = v2.iter_mut();

        assert_eq!(v2_iter.next(), Some(&mut 1));
        assert_eq!(v2_iter.next(), Some(&mut 2));
        assert_eq!(v2_iter.next(), Some(&mut 3));
        assert_eq!(v2_iter.next(), None);

        println!("{:?}", v2);
    }

    #[test]
    fn works_3() {
        let v3 = vec![1, 2, 3];
        let mut v3_iter = v3.into_iter();

        assert_eq!(v3_iter.next(), Some(1));
        assert_eq!(v3_iter.next(), Some(2));
        assert_eq!(v3_iter.next(), Some(3));
        assert_eq!(v3_iter.next(), None);

        // println!("{:?}", v3);
    }

    #[test]
    fn works_4() {
        let v = &[1, 2, 3];
        let mut v_iter = v.iter();

        assert_eq!(v_iter.next(), Some(&1));
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&3));
        assert_eq!(v_iter.next(), None);

        println!("{:?}", v);
    }

    #[test]
    fn works_5() {
        let v = &mut [1, 2, 3];
        let mut v_iter = v.iter_mut();

        assert_eq!(v_iter.next(), Some(&mut 1));
        assert_eq!(v_iter.next(), Some(&mut 2));
        assert_eq!(v_iter.next(), Some(&mut 3));
        assert_eq!(v_iter.next(), None);

        println!("{:?}", v);
    }

    #[test]
    fn works_6() {
        let v = [1, 2, 3];
        let mut v_iter = v.into_iter();

        assert_eq!(v_iter.next(), Some(1));
        assert_eq!(v_iter.next(), Some(2));
        assert_eq!(v_iter.next(), Some(3));
        assert_eq!(v_iter.next(), None);

        println!("{:?}", v);
    }
}
