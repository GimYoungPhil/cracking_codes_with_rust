use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn hello() -> &'static str {
    "RUST"
}

fn hello2() -> &'static str {
    let s = "RUST";

    s
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let i = ImportantExcerpt {
            part: &novel,
        };

        println!("novel: {}", i.part);
    }

    #[test]
    fn works_2() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let i = ImportantExcerpt {
            part: &novel,
        };

        println!("novel level : {}", i.level());
    }

    #[test]
    fn works_3() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let i = ImportantExcerpt {
            part: &novel,
        };

        println!("novel announce : {}", i.announce_and_return_part("Now start!"));
    }

    #[test]
    fn works_4() {
        println!("hello {}", hello());
    }

    #[test]
    fn works_5() {
        let x = "abc";
        let y = "abcde";
        let long = longest_with_an_announcement(x, y, '~');

        println!("long: {}", long);
    }

}
