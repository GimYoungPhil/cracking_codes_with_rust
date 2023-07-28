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

        let user_2: Option<ShirtColor> = None;
        let giveaway_2 = store.giveaway(user_2);
        println!("{:?}: {:?}", user_2, giveaway_2);
    }
}
