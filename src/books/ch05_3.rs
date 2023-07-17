#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn addWidth(&mut self, value: u32) {
        self.width += value;
    }

    fn duble(self) -> Rectangle {
        Rectangle {
            width: self.width * 2,
            height: self.height * 2,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_0() {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        assert_eq!(rect.area(), 1500_u32);
    }

    #[test]
    fn works_1() {
        let mut rect = Rectangle {
            width: 30,
            height: 50,
        };

        rect.addWidth(100);

        assert_eq!(rect.width, 130);
    }

    #[test]
    fn works_2() {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        let rect_duble = rect.duble();

        assert_eq!(rect_duble.width, 60);
    }

    #[test]
    fn works_3() {
        let rect_0 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect_1 = Rectangle {
            width: 25,
            height: 40,
        };
        let rect_2 = Rectangle {
            width: 33,
            height: 30,
        };

        assert_eq!(rect_0.can_hold(&rect_1), true);
        assert_eq!(rect_0.can_hold(&rect_2), false);
    }

    #[test]
    fn works_4() {
        let rect = Rectangle::square(50);

        assert_eq!(rect.area(), 2500_u32);
    }
}
