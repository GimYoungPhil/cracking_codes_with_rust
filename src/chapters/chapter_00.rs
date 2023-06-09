pub fn some_0() {
    let integers: [(i8, i8); 4] = [
        (13_i8, 66_i8),
        (13_i8, -66_i8),
        (-13_i8, 66_i8),
        (-13_i8, -66_i8),
    ];
    display_div(&integers);
    display_div_euclid(&integers);
}

pub fn some_1() {
    let integers: [(i8, i8); 4] = [
        (79_i8, 66_i8),
        (79_i8, -66_i8),
        (-79_i8, 66_i8),
        (-79_i8, -66_i8),
    ];
    display_div(&integers);
    display_div_euclid(&integers);
}

pub fn some_2() {
    let integers: [(i8, i8); 4] = [
        (i8::MAX, 1_i8),
        (i8::MAX, -1_i8),
        (i8::MIN, 1_i8),
        (i8::MIN, -1_i8),
    ];
    display_checked_div(&integers);
    display_checked_div_euclid(&integers);
}

pub fn display_div(integers: &[(i8, i8)]) {
    for (dividend, rhs) in integers {
        println!("{:>4} / {:>4} = {:>4}", dividend, rhs, dividend / rhs);
    }

    for (dividend, rhs) in integers {
        println!("{:>4} % {:>4} = {:>4}", dividend, rhs, dividend % rhs);
    }
}

pub fn display_div_euclid(integers: &[(i8, i8)]) {
    for (dividend, rhs) in integers {
        println!("{:>4} div_euclid {:>4} = {:>4}", dividend, rhs, dividend.div_euclid(*rhs));
    }

    for (dividend, rhs) in integers {
        println!("{:>4} rem_euclid {:>4} = {:>4}", dividend, rhs, dividend.rem_euclid(*rhs));
    }
}

pub fn display_checked_div(integers: &[(i8, i8)]) {
    for (dividend, rhs) in integers {
        print!("{:>4} div {:>4} =", dividend, rhs);
        match dividend.checked_div(*rhs) {
            Some(r) => print!("{:>4}", r),
            None => print!("{}", "panic"),
        }
        println!();
    }

    for (dividend, rhs) in integers {
        print!("{:>4} rem {:>4} =", dividend, rhs);
        match dividend.checked_rem(*rhs) {
            Some(r) => print!("{:>4}", r),
            None => print!("{}", "panic"),
        }
        println!();
    }
}

pub fn display_checked_div_euclid(integers: &[(i8, i8)]) {
    for (dividend, rhs) in integers {
        print!("{:>4} div {:>4} =", dividend, rhs);
        match dividend.checked_div_euclid(*rhs) {
            Some(r) => print!("{:>4}", r),
            None => print!("{}", "panic"),
        }
        println!();
    }

    for (dividend, rhs) in integers {
        print!("{:>4} rem {:>4} =", dividend, rhs);
        match dividend.checked_rem_euclid(*rhs) {
            Some(r) => print!("{:>4}", r),
            None => print!("{}", "panic"),
        }
        println!();
    }
}

pub fn display_rem_0(d: i8) {
    let rhs_list: [i8; 6] = [127, 1, 0, -1, -127, -128];

    println!("|{:_^7}|{:_^7}|{:_^8}|{:_^7}|{:_^7}|{:_^7}|", "d", "d", "self", "q", "rhs", "r");
    for rhs in rhs_list {
        match d.checked_div_euclid(rhs) {
            Some(q) => {
                println!("|{:^7}/{:^7}|{:^7}>={:^7}*{:^7}+{:^7}|", d, rhs, d, q, rhs, d.rem_euclid(rhs));
            },
            None => {
                println!("|{:^7}/{:^7}|{:^32}|", d, rhs, "panic !");
            },
        }
    }
    println!();
}


pub fn display_rem_1(d: i8) {
    println!("|{:_>7}___|{:_>7}___|{:_>7}___|{:_>7}___|{:_>7}___|", d, "div", "euclid", "rem", "euclid");
    for rhs in i8::MIN..=i8::MAX {
        print!("|{:>7}   |", rhs);
        match d.checked_div(rhs) {
            Some(r) => print!("{:>7}   |", r),
            None => print!("{:>7}   |", "panic"),
        }
        match d.checked_div_euclid(rhs) {
            Some(r) => print!("{:>7}   |", r),
            None => print!("{:>7}   |", "panic"),
        }
        match d.checked_rem(rhs) {
            Some(r) => print!("{:>7}   |", r),
            None => print!("{:>7}   |", "panic"),
        }
        match d.checked_rem_euclid(rhs) {
            Some(r) => print!("{:>7}   |", r),
            None => print!("{:>7}   |", "panic"),
        }
        println!();
    }
} 

#[cfg(test)]
mod tests {
    use std::ops::Rem;

    #[test]
    fn it_works_0() {
        assert_eq!(127_i8.rem(-128), 127);
        assert_eq!(127_i8.rem_euclid(-128), 127);
    }

    #[test]
    fn it_works_i8_0() {
        assert_eq!(127_i8.div_euclid(127), 1);
        assert_eq!(127_i8.rem_euclid(127), 0);

        assert_eq!(127_i8.div_euclid(1), 127);
        assert_eq!(127_i8.rem_euclid(1), 0);

        // panic
        // assert_eq!(127_i8.div_euclid(0), 0);
        // assert_eq!(127_i8.rem_euclid(0), 0);

        assert_eq!(127_i8.div_euclid(-1), -127);
        assert_eq!(127_i8.rem_euclid(-1), 0);

        assert_eq!(127_i8.div_euclid(-127), -1);
        assert_eq!(127_i8.rem_euclid(-127), 0);

        assert_eq!(127_i8.div_euclid(-128), 0);
        assert_eq!(127_i8.rem_euclid(-128), 127);
    }

    #[test]
    #[should_panic]
    fn it_works_i8_panic() {
        assert_eq!(127_i8.div_euclid(0), 0);
        assert_eq!(127_i8.rem_euclid(0), 0);

        assert_eq!(1_i8.div_euclid(0), 0);
        assert_eq!(1_i8.rem_euclid(0), 0);

        assert_eq!(0_i8.div_euclid(0), 0);
        assert_eq!(0_i8.rem_euclid(0), 0);

        assert_eq!((-1_i8).div_euclid(0), 0);
        assert_eq!((-1_i8).rem_euclid(0), 0);
    }

    #[test]
    fn it_works_i8_1() {
        assert_eq!(1_i8.div_euclid(127), 0);
        assert_eq!(1_i8.rem_euclid(127), 1);

        assert_eq!(1_i8.div_euclid(1), 1);
        assert_eq!(1_i8.rem_euclid(1), 0);

        // panic
        // assert_eq!(1_i8.div_euclid(0), 0);
        // assert_eq!(1_i8.rem_euclid(0), 0);

        assert_eq!(1_i8.div_euclid(-1), -1);
        assert_eq!(1_i8.rem_euclid(-1), 0);

        assert_eq!(1_i8.div_euclid(-127), 0);
        assert_eq!(1_i8.rem_euclid(-127), 1);

        assert_eq!(1_i8.div_euclid(-128), 0);
        assert_eq!(1_i8.rem_euclid(-128), 1);
    }

    #[test]
    fn it_works_i8_2() {
        assert_eq!(0_i8.div_euclid(127), 0);
        assert_eq!(0_i8.rem_euclid(127), 0);

        assert_eq!(0_i8.div_euclid(1), 0);
        assert_eq!(0_i8.rem_euclid(1), 0);

        // panic
        // assert_eq!(0_i8.div_euclid(0), 0);
        // assert_eq!(0_i8.rem_euclid(0), 0);

        assert_eq!(0_i8.div_euclid(-1), 0);
        assert_eq!(0_i8.rem_euclid(-1), 0);

        assert_eq!(0_i8.div_euclid(-127), 0);
        assert_eq!(0_i8.rem_euclid(-127), 0);

        assert_eq!(0_i8.div_euclid(-128), 0);
        assert_eq!(0_i8.rem_euclid(-128), 0);
    }

    #[test]
    fn it_works_i8_3() {
        assert_eq!((-1_i8).div_euclid(127), -1);
        assert_eq!((-1_i8).rem_euclid(127), 126);

        assert_eq!((-1_i8).div_euclid(1), -1);
        assert_eq!((-1_i8).rem_euclid(1), 0);

        // panic
        // assert_eq!((-1_i8).div_euclid(0), 0);
        // assert_eq!((-1_i8).rem_euclid(0), 0);

        assert_eq!((-1_i8).div_euclid(-1), 1);
        assert_eq!((-1_i8).rem_euclid(-1), 0);

        assert_eq!((-1_i8).div_euclid(-127), 1);
        assert_eq!((-1_i8).rem_euclid(-127), 126);

        assert_eq!((-1_i8).div_euclid(-128), 1);
        assert_eq!((-1_i8).rem_euclid(-128), 127);
    }

    #[test]
    fn it_works_i8_4() {
        assert_eq!((-127_i8).div_euclid(127), -1);
        assert_eq!((-127_i8).rem_euclid(127), 0);

        assert_eq!((-127_i8).div_euclid(1), -127);
        assert_eq!((-127_i8).rem_euclid(1), 0);

        // panic
        // assert_eq!((-127_i8).div_euclid(0), 0);
        // assert_eq!((-127_i8).rem_euclid(0), 0);

        assert_eq!((-127_i8).div_euclid(-1), 127);
        assert_eq!((-127_i8).rem_euclid(-1), 0);

        assert_eq!((-127_i8).div_euclid(-127), 1);
        assert_eq!((-127_i8).rem_euclid(-127), 0);

        assert_eq!((-127_i8).div_euclid(-128), 1);
        assert_eq!((-127_i8).rem_euclid(-128), 1);
    }

    #[test]
    fn it_works_i8_5() {
        assert_eq!((-128_i8).div_euclid(127), -2);
        assert_eq!((-128_i8).rem_euclid(127), 126);

        assert_eq!((-128_i8).div_euclid(1), -128);
        assert_eq!((-128_i8).rem_euclid(1), 0);

        // panic
        // assert_eq!((-128_i8).div_euclid(0), 0);
        // assert_eq!((-128_i8).rem_euclid(0), 0);

        // assert_eq!((-128_i8).div_euclid(-1), i8::MAX + 1);
        // assert_eq!((-128_i8).rem_euclid(-1), 0);

        assert_eq!((-128_i8).div_euclid(-127), 2);
        assert_eq!((-128_i8).rem_euclid(-127), 126);

        assert_eq!((-128_i8).div_euclid(-128), 1);
        assert_eq!((-128_i8).rem_euclid(-128), 0);
    }
}
