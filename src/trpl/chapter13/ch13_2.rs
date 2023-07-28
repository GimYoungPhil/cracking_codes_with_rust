fn  add_one_v1(x: u32) -> u32 { x + 1 }

fn ex13_2() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    let x1 = add_one_v1(1);
    let x2 = add_one_v2(1);
    let x3 = add_one_v3(1i16);
    let x4 = add_one_v4(1u8);

    println!("{x1}, {x2}, {x3}, {x4}");
}

fn ex13_3() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    println!("{s}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        ex13_2();
        ex13_3();
    }
}
