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

fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Notify news! {}, {}", item1.summarize(), item2.summarize());
}

fn breaking<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let fb = Twitter {
            message: String::from("new Twitter"),
        };
        let ins = Instagram {
            content: String::from("new Instagram"),
        };
        let fb2 = Twitter {
            message: String::from("new Twitter2"),
        };
        let ins2 = Instagram {
            content: String::from("new Instagram2"),
        };

        notify(&fb, &ins);
        breaking(&fb, &fb2);
        breaking(&ins, &ins2);
    }
}
