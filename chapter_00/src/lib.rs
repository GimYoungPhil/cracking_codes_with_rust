pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn display_division() {
    let integers = [
        (13_i8, 5_i8),
        (13_i8, -5_i8),
        (-13_i8, 5_i8),
        (-13_i8, -5_i8),
    ];
    for (dividend, rhs) in integers {
        println!("{} / {} = {}", dividend, rhs, dividend / rhs);
    }

    for (dividend, rhs) in integers {
        println!("{} % {} = {}", dividend, rhs, dividend % rhs);
    }
}

pub fn display_euclid() {
    let integers = [
        (13_i8, 5_i8),
        (13_i8, -5_i8),
        (-13_i8, 5_i8),
        (-13_i8, -5_i8),
    ];
    for (dividend, rhs) in integers {
        println!("{} div_euclid {} = {}", dividend, rhs, dividend.div_euclid(rhs));
    }

    for (dividend, rhs) in integers {
        println!("{} rem_euclid {} = {}", dividend, rhs, dividend.rem_euclid(rhs));
    }
}

pub fn euclid(d: i8) {
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


pub fn display_rem(d: i8) {
    println!("|{:_>7}___|{:_>7}___|{:_>7}___|{:_>7}___|{:_>7}___|", d, "div", "euclid", "rem", "euclid");
    for rhs in i8::MIN..=i8::MAX {
        print!("|{:>7}   |", rhs);
        match d.checked_div(rhs) {
            Some(r) => print!("{:>7}   |", r),
            None => print!("{:>7}   |","panic"),
        }
        match d.checked_div_euclid(rhs) {
            Some(r) => print!("{:>7}   |", r),
            None => print!("{:>7}   |","panic"),
        }
        match d.checked_rem(rhs) {
            Some(r) => print!("{:>7}   |", r),
            None => print!("{:>7}   |","panic"),
        }
        match d.checked_rem_euclid(rhs) {
            Some(r) => print!("{:>7}   |", r),
            None => print!("{:>7}   |","panic"),
        }
        println!();
    }
} 

#[cfg(test)]
mod tests {
    use std::ops::Rem;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_0() {
        assert_eq!(127_i8.rem(-128), 127);
        assert_eq!(127_i8.rem_euclid(-128), 127);
    }
}
