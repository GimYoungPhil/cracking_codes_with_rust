struct Facebook {
    text: String,
}

struct Tiktok {
    title: String,
}

trait Summary {
    fn summarize(&self) -> String {
        format!("Read ...")
    }
}

impl Summary for Facebook {
    fn summarize(&self) -> String {
        format!("FACEbook: {}", self.text)
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
