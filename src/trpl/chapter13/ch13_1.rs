#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        ShirtColor::Blue
    }

    fn giveaway_1(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or(self.best_color())
    }

    fn best_color(&self) -> ShirtColor {
        ShirtColor::Red
    }
}

mod one {
    enum Optbody<T> {
        Somebody(T),
        Nobody,
    }

    impl<T> Optbody<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Optbody::Somebody(x) => x,
                Optbody::Nobody => f(),
            }
        }
    }

    enum Console {
        Playstation5,
        Switch,
        XboxSeriesX,
    }

    struct Inven {
        consoles: Vec<Console>
    }

    fn getXbox() -> Console {
        Console::XboxSeriesX
    }

    impl Inven {
        fn giveaway(&self, user_prefer: Optbody<Console>) -> Console {
            user_prefer.unwrap_or_else(|| self.most_console())
        }

        fn most_console(&self) -> Console {
            Console::Switch
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let store = Inventory {
            shirts: vec![],
        };

        let user_1 = Some(ShirtColor::Red);
        let giveaway_1 = store.giveaway(user_1);
        println!("{:?}: {:?}", user_1, giveaway_1);

        let user_2: Option<ShirtColor> = Option::None;
        let giveaway_2 = store.giveaway(user_2);
        println!("{:?}: {:?}", user_2, giveaway_2);
    }
}
