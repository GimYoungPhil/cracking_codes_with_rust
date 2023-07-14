#[cfg(test)]
mod tests {

    #[test]
    fn work_0() {
        let x = 5;
        let y = x;

        println!("x = {}", x);
        println!("y = {}", y);
        println!("work 0");
    }

    #[test]
    fn work_1() {
        let x = 5;
        let mut y = x;

        y += 1;

        println!("x = {}", x);
        println!("y = {}", y);
        println!("work 1");
    }

     #[test]
    fn work_2() {
        let s = String::from("hello");
        let ms = s;

        println!("{}", ms);
        println!("work 2");
    }

     #[test]
    fn work_3() {
        let s = String::from("hello");
        let mut ms = s;

        ms.push_str(", world!");

        println!("{}", ms);
        println!("work 3");
    }

    #[test]
    fn work_4() {
        let s = String::from("hello");
        let bs = &s;

        println!("{}", s);
        println!("{}", bs);
        println!("work 4");
    }

    #[test]
    fn work_5() {
        let mut s = String::from("hello");
        let bs = &mut s;

        bs.push_str(", world!");

        println!("{}", bs);
        println!("work 5");
    }

    #[test]
    fn work_6() {
        let s = String::from("hello");
        let ss: &str = &s[..];

        println!("{}", s);
        println!("{}", ss);
        println!("work 6");
    }

    #[test]
    fn work_7() {
        let mut s = String::from("hello");
        let ss = &mut s[..];

        ss.make_ascii_uppercase();

        println!("{}", ss);
        println!("work 7");
    }

    #[test]
    fn work_8() {
        let s = "hello";
        let ss = &s[1..4];

        println!("{}", s);
        println!("{}", ss);
        println!("work 8");
    }

    // #[test]
    // fn work_9() {
    //     let mut s = String::from("hello");

    //     let r1 = &s;
    //     let r2 = &s;
    //     let r3 = &mut s;

    //     println!("{}, {}", r1, r2);
    //     println!("{}", r3);

    //     println!("work 9");
    // }

    #[test]
    fn work_10() {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{}, {}", r1, r2);

        let r3 = &mut s;
        println!("{}", r3);

        println!("work 10");
    }
}
