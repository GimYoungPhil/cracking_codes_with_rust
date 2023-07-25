struct Twitter {
    message: String,
}

struct Instagram {
    content: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Twitter {
    fn summarize(&self) -> String {
        format!("hello {}", self.message)
    }
}

impl Summary for Instagram {
    fn summarize(&self) -> String {
        format!("hi {}", self.content)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let tw: Twitter = Twitter {
            message: String::from("newTwwiter"),
        };
        let ins = Instagram {
            content: String::from("newInstagram"),
        };

        type Alli<T: Summary> = Vec<T>;



        for item in &list {
            println!("{}", item.summarize());
        }
    }
}
