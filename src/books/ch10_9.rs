struct Facebook {
    text: String,
}

struct Tiktok {
    title: String,
}

trait Summary {
    fn summarize_author(&self) -> String {
        format!("@Author...")
    }

    fn summarize(&self) -> String {
        format!("@Read more from {}", self.summarize_author())
    }
}

impl Summary for Facebook {
    fn summarize_author(&self) -> String {
        format!("{}", self.text)
    }
}

impl Summary for Tiktok {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let fb = Facebook {
            text: String::from("new Facebook"),
        };
        let tic = Tiktok {
            title: String::from("new Tiktok"),
        };

        println!("{}", fb.summarize());
        println!("{}", tic.summarize());
    }
}
