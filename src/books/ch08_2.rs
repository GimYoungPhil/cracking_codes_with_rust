fn ex_0() {
    let hello = "Здравствуйте";
    let f = &hello[0..2];
    let s = &hello[2..4];

    println!("{} .. {}", f, s);
}

fn ex_1() {
    let hello = "Здравствуйте";
    let f = &hello[0..4];

    println!("{}", f);
}

fn ex_2() {
    let hello = "Здравствуйте";

    for c in hello.chars() {
        println!("{}", c);
    }
}

fn ex_3() {
    let hello = "Здравствуйте";

    for b in hello.bytes() {
        println!("{}", b);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_0() {
        // ex_0();
        ex_1();
        // ex_2();
        // ex_3();
    }

}
