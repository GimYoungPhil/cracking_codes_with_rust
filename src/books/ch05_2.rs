fn area_basic(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimesions: (u32, u32)) -> u32 {
    dimesions.0 * dimesions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_0() {
        let width = 30;
        let height = 50;
        assert_eq!(area_basic(width, height), 1500_u32);
    }

    #[test]
    fn works_1() {
        let rect = (30, 50);
        assert_eq!(area_tuple(rect), 1500_u32);
    }

    #[test]
    fn works_2() {
        let reactangle = Rectangle {
            width: 30,
            height: 50,
        };
        assert_eq!(area_struct(&reactangle), 1500_u32);
    }
}
