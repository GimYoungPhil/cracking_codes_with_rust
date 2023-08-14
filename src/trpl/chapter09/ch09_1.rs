use std::{fs::File, io::ErrorKind};

fn ex_0() {
    panic!("crash and burn");
}

fn ex_1() {
    let v = vec![1, 2, 3];

    v[99];
}

fn ex_3() {
    let greeting_file_result = File::open("hello.txt");
}

fn ex_4() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn ex_5() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other => {
                panic!("Problem opening the file: {}", other);
            },
        },
    };
}

fn ex_6() {
    let greeting_file = File::open("hello.txt").unwrap();
}

fn ex_7() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

use std::io::{self, Read};

fn ex_8() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn ex_9() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn ex10() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

use std::fs;

fn ex11() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn works_0() {
        ex_0();
    }

    #[test]
    #[should_panic]
    fn works_1() {
        ex_1();
    }
}
