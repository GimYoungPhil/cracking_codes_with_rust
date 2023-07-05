fn move_integer() {
    let x = 10;
    let y = x;

    println!("{x}");
    println!("{y}");
}

fn move_array() {
    let x = [1, 2, 3, 4, 5];
    let y = x;

    println!("{:?}", x);
    println!("{:?}", y);
}

fn move_tuple() {
    let x = (1, 2);
    let y = x;

    println!("{:?}", x);
    println!("{:?}", y);
}

fn move_vec() {
    let x = vec![1, 2, 3, 4, 5];
    let y = x;

    // println!("{:?}", x);
    println!("{:?}", y);
}

fn move_str() {
    let x = "hello";
    let y = x;

    println!("{}", x);
    println!("{}", y);
}

fn move_string() {
    let x = String::from("hello");
    let y = x;

    // println!("{}", x);
    println!("x is...");
    println!("{}", y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_0() {
        move_integer();
    }

    #[test]
    fn works_1() {
        move_array();
    }

    #[test]
    fn works_2() {
        move_tuple();
    }

    #[test]
    fn works_3() {
        move_vec();
    }

    #[test]
    fn works_4() {
        move_str();
    }

    #[test]
    fn works_5() {
        move_string();
    }
}
