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

trait Display {
    fn display(&self) -> String {
        format!("hi")
    }
}

impl Display for Twitter {}

impl Display for Instagram {}

fn notify(item: &(impl Summary + Display)) {
    println!("Notify news! {}, {}", item.summarize(), item.display());
}

fn breaking<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}, {}", item.summarize(), item.display());
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

        notify(&fb);
        breaking(&ins);
    }
}
