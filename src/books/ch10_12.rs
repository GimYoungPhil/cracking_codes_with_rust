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

fn return_summarizable() -> impl Summary {
    Twitter {
        message: String::from("Twitter"),
    }
}

// fn return_instagram<T: Summary>() -> T {
//     Instagram {
//         content: String::from("Instagram")
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let some = return_summarizable();

        println!("{}", some.summarize());
    }
}
