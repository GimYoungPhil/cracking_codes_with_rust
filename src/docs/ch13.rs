#[derive(Debug, PartialEq)]
enum ShirtColor {
    Red,
    Green,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn getaway(&self, user_info: Option<ShirtColor>) -> ShirtColor {
        // let clo0 = |x: i32| -> i32 { x + 1};
        // let clo1 = || self.always_red();
        user_info.unwrap_or_else(|| self.always_red())
    }

    fn always_red(&self) -> ShirtColor {
        ShirtColor::Red
    }

    fn always_blue(&self) -> ShirtColor {
        ShirtColor::Blue
    }
}

fn ff() {
    let example_closuer = |x| x;
    // let s = example_closuer(String::from("hello"));
    let n = example_closuer(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inventory = Inventory {
            shirts: vec![
                ShirtColor::Blue,
                ShirtColor::Red,
                ShirtColor::Red,
                ShirtColor::Red,
                ShirtColor::Blue,
            ],
        };

        let user = Some(ShirtColor::Red);
        assert_eq!(inventory.getaway(user), ShirtColor::Red);

        let user = Some(ShirtColor::Green);
        assert_eq!(inventory.getaway(user), ShirtColor::Green);

        let user = Some(ShirtColor::Blue);
        assert_eq!(inventory.getaway(user), ShirtColor::Blue);

        let user = None;
        assert_eq!(inventory.getaway(user), ShirtColor::Red);
    }
}
