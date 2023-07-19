fn greeting(name: &str) {
    println!("Hello, {name}!");
}

fn date_string() -> String {
    let date = String::from("2023-07-19");
    date
}

fn time_string() -> &'static str {
    "AM 10:23:47"
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
