mod hello {
    fn hello_0() {
        let words = String::from("hello");
        println!("{}", words);
    }

    fn hello_1() -> String {
        let words = String::from("hello");
        println!("{}", words);

        words
    }

    fn hello_2() -> &'static String {
        let words = String::from("hello");
        println!("{}", words);

        &words
    }

    fn hello_3() -> &'static str {
        let words = "hello";
        println!("{}", words);

        words
    }
}

// mod hello_2 {
//     fn math() -> &'static i32 {
//         let i = 10;
//         &i
//     }

//     fn hello_0() -> &str {
//         let words = String::from("hello");
//         println!("{}", words);

//         &words;
//     }

//     fn hello_1(name: String) -> &str {
//         println!("{}", name);

//         &name;
//     }

//     fn hello_2(mut name: String) -> &str {
//         name.push_str(", hi");

//         &name;
//     }
// }


mod hello_3 {
    fn hello(name: String) {
        let words = String::from("hello");
        println!("{}, {}", words, name);
    }

    fn hello_str_0(name: String) -> String {
        let mut words = String::from("hello");
        println!("{}, {}", words, name);

        words.push_str(&name);

        words
    }

    fn hello_str_1(mut name: String) -> String {
        let words = String::from("hello");
        println!("{}, {}", words, name);

        name.push_str(&words);

        name
    }
}

fn greeting(name: &str) {
    println!("Hello, {name}!");
}

struct Simple {
    name: String,
}

impl Simple {
    fn greeting_str(&self) -> &str {
        "hi"
    }

    fn greeting_string(&self) -> String {
        String::from("Hi")
    }

    fn hello(&self, greeting: &mut String) {
        greeting.push_str(&self.name);
    }

    fn hello_str(&mut self) -> &str {
        self.name.push_str(", Hello");
        
        &self.name
    }

    fn hello_string(&self) -> String {
        let mut hello = String::from("Hello");
        hello.push_str(&self.name);

        hello
    }

    fn hello_string_2(&self, mut greeting: String) -> String {
        greeting.push_str(&self.name);

        greeting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        greeting("Sue");
    }
}
