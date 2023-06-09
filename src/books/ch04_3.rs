// fn first_word_index(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn run_0() {
//     let mut s = String::from("hello world");

//     let word = first_word_index(&s);

//     s.clear();

//     println!("the first word is: {}", word);
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn run_1() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     s.clear();

//     println!("the first word is: {}", word);
// }

fn largest_number(a: &[i32; 5]) -> i32 {
    let mut b = a[0];

    for &item in a {
        if item > b {
            b = item;
        }
    }

    b
}

fn run_2() {

    let largest;

    {
        let a: [i32; 5] = [10, 30, 99, 100, 98];
        largest = largest_number(&a);
    }

    println!("{largest}");
}

fn largest_number_2(a: &[i32; 5]) -> &i32 {
    let mut b = &a[0];

    for item in a {
        if item > b {
            b = item;
        }
    }

    b
}

fn run_3() {

    // let largest;

    {
        let a: [i32; 5] = [10, 30, 99, 100, 98];
        // largest = largest_number_2(&a);
    }

    // println!("{largest}");
}

fn bigger_number(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else if x < y {
        y
    } else {
        0
    }
}

fn run_4() {

    let bigger;

    {
        let a = 10;
        let b = 9;
        bigger = bigger_number(a, b);
    }

    println!("{bigger}");
}

fn bigger_number_2<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else if x < y {
        y
    } else {
        &0
    }
}

fn run_5() {

    // let bigger;

    // {
    //     let a = 10;
    //     let b = 9;
    //     bigger = bigger_number_2(&a, &b);
    // }

    // println!("{bigger}");
}

fn run_6() {

    // let a = 10;
    // let mut b = 9;
    // let bigger = bigger_number_2(&a, &b);

    // println!("{bigger}");
    // b = 99;
    // println!("{bigger}");
}

// fn iterator_0() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     for v in a {
//         println!("v: {}", v);
//     }

//     println!("a: {:?}", a);
// }

// fn iterator_1() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     for v in &a {
//         println!("v: {}", v);
//     }

//     println!("a: {:?}", a);
// }

// fn iterator_2() {
//     let mut a: [i32; 5] = [1, 2, 3, 4, 5];
//     for v in &mut a {
//         println!("v: {}", v);
//     }

//     println!("a: {:?}", a);
// }

// fn iterator_3() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     for v in a.into_iter() {
//         println!("v: {}", v);
//     }

//     println!("a: {:?}", a);
// }

// fn iterator_4() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     for v in a.iter() {
//         println!("v: {}", v);
//     }

//     println!("a: {:?}", a);
// }

// fn iterator_5() {
//     let mut a: [i32; 5] = [1, 2, 3, 4, 5];
//     for v in a.iter_mut() {
//         println!("v: {}", v);
//     }

//     println!("a: {:?}", a);
// }

// fn iterator_6() {
//     let mut a: [i32; 5] = [1, 2, 3, 4, 5];
//     for v in a {
//         println!("v: {}", v);
//     }

//     for v in &a {
//         println!("v: {}", v);
//     }

//     for v in &mut a {
//         *v += 10;
//         println!("v: {}", v);
//     }

//     println!("a: {:?}", a);
// }

// fn iterator_7() {
//     let mut a: [i32; 5] = [1, 2, 3, 4, 5];
//     for v in a.into_iter() {
//         println!("v: {}", v);
//     }

//     for v in a.iter() {
//         println!("v: {}", v);
//     }

//     for v in a.iter_mut() {
//         *v += 10;
//         println!("v: {}", v);
//     }

//     println!("a: {:?}", a);
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn work_0() {
    //     iterator_0();
    //     iterator_1();
    //     iterator_2();
    //     iterator_3();
    //     iterator_4();
    //     iterator_5();
    // }

    // #[test]
    // fn work_1() {
    //     iterator_6();
    //     iterator_7();
    // }
}
