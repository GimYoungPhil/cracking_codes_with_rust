fn ex_8_1() {
    let v: Vec<i32> = Vec::new();
}

fn ex_8_2() {
    let v = vec![1, 2, 3];
}

fn ex_8_3() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn ex_8_4() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }
}

fn ex_8_5() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

fn ex_8_6() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    println!("The first element is {}", first);

    v.push(6);
}

fn ex_8_7() {
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }
}

fn ex_8_8() {
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);
}

fn ex_8_9() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn works_0() {
        ex_8_5();
    }

    #[test]
    fn works_1() {
        ex_8_9();
    }
}
