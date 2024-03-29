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

pub struct Tiktok {
    pub title: String,
}

fn notify(item: &impl Summary) {
    println!("Notify news! {}", item.summarize());
}

fn breaking<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}


#[cfg(test)]
mod tests {
    use super::{Twitter, Instagram, Summary, notify};

    #[test]
    fn works_1() {
        let tw: Twitter = Twitter {
            message: String::from("new Twwiter"),
        };
        let ins = Instagram {
            content: String::from("new Instagram"),
        };

        notify(&tw);
        notify(&ins);
    }
}
