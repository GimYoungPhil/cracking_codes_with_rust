#[derive(Debug, PartialEq)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&mut self, user: Option<ShirtColor>) -> ShirtColor {
        // user.unwrap_or_else(|| self.most_stocked())
        // user.unwrap_or_else(|| self.always_red())
        user.unwrap_or_else(|| self.shirts.pop().unwrap() )
    }

    fn always_red(&self) -> ShirtColor {
        ShirtColor::Red
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red],
        };

        let user_1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_1);
        assert_eq!(ShirtColor::Red, giveaway1);

        let user_2 = Some(ShirtColor::Blue);
        let giveaway2 = store.giveaway(user_2);
        assert_eq!(ShirtColor::Blue, giveaway2);

        let user_3: Option<ShirtColor> = None;
        let giveaway3 = store.giveaway(user_3);
        assert_eq!(ShirtColor::Red, giveaway3);

        let user_4: Option<ShirtColor> = None;
        let giveaway4 = store.giveaway(user_4);
        assert_eq!(ShirtColor::Red, giveaway4);

        let user_5: Option<ShirtColor> = None;
        let giveaway5 = store.giveaway(user_5);
        assert_eq!(ShirtColor::Blue, giveaway5);
    }
}
