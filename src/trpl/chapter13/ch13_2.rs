fn  add_one_v1(x: u32) -> u32 { x + 1 }

fn ex13_1() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    let x1 = add_one_v1(1);
    let x2 = add_one_v2(1);
    let x3 = add_one_v3(1i16);
    let x4 = add_one_v4(1u8);

    println!("{x1}, {x2}, {x3}, {x4}");
}

fn ex13_2() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let s = expensive_closure(10);
    println!("{s}");
}

fn ex13_3() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    println!("{s}");
}

fn ex13_4() {
    let list = vec![1, 2, 3, 4, 5];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn ex13_5() {
    let mut list = vec![1, 2, 3, 4, 5];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

use std::{thread, time::Duration};

fn ex13_6() {
    let list: Vec<i32> = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // println!("{:?}", list);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_2() {
        ex13_2();
    }

    #[test]
    fn works_3() {
        ex13_3();
    }

    #[test]
    fn works_4() {
        ex13_4();
    }

    #[test]
    fn works_5() {
        ex13_5();
    }

    #[test]
    fn works_6() {
        ex13_6();
    }

}
