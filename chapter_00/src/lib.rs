pub fn add(left: usize, right: usize) -> usize {
    left + right
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
            }
        }
    }
    println!();
}

pub fn display_rem(d: i8) {
    for rhs in i8::MIN..i8::MAX {
        match d.checked_rem(rhs) {
            Some(r) => {
                println!("{:} | {:} | {:}", d, d.checked_rem(rhs).unwrap(), d.checked_rem_euclid(rhs).unwrap());
            },
            None => {
                println!("{} | {}", d, "panic");
            }
        }
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
