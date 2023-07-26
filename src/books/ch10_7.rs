use crate::books::ch10_6::{Summary, Tiktok};

struct Facebook {
    text: String,
}

impl Summary for Facebook {
    fn summarize(&self) -> String {
        format!("FACEbook: {}", self.text)
    }
}

impl Summary for Tiktok {
    fn summarize(&self) -> String {
        format!("tiktok:: {}", self.title)
    }
}

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
