fn iterator_0() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for v in a {
        println!("v: {}", v);
    }

    println!("a: {:?}", a);
}

fn iterator_1() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for v in &a {
        println!("v: {}", v);
    }

    println!("a: {:?}", a);
}

fn iterator_2() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    for v in &mut a {
        println!("v: {}", v);
    }

    println!("a: {:?}", a);
}

fn iterator_3() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for v in a.into_iter() {
        println!("v: {}", v);
    }

    println!("a: {:?}", a);
}

fn iterator_4() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for v in a.iter() {
        println!("v: {}", v);
    }

    println!("a: {:?}", a);
}

fn iterator_5() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    for v in a.iter_mut() {
        println!("v: {}", v);
    }

    println!("a: {:?}", a);
}

fn iterator_6() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    for v in a {
        println!("v: {}", v);
    }

    for v in &a {
        println!("v: {}", v);
    }

    for v in &mut a {
        *v += 10;
        println!("v: {}", v);
    }

    println!("a: {:?}", a);
}

fn iterator_7() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    for v in a.into_iter() {
        println!("v: {}", v);
    }

    for v in a.iter() {
        println!("v: {}", v);
    }

    for v in a.iter_mut() {
        *v += 10;
        println!("v: {}", v);
    }

    println!("a: {:?}", a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_0() {
        iterator_0();
        iterator_1();
        iterator_2();
        iterator_3();
        iterator_4();
        iterator_5();
    }

    #[test]
    fn work_1() {
        iterator_6();
        iterator_7();
    }
}
