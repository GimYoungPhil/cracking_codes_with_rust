pub trait Area<T> {
    fn calculate_area(&self) -> T;
}

struct Triangle<T> {
    width: T,
    height: T,
}

impl<T> Area<T> for Triangle<T> {
    fn calculate_area(&self) -> T {
        let result = (self.width) * (self.height);
        result
    }
}

struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Area<T> for Rectangle<T> {
    fn calculate_area(&self) -> T {
        let result = (self.width) * (self.height);
        result
    }
}

fn display<A: Area>(list: &[A]) {
    for item in list {
        let area = item.calculate_area();
        println!("Area: {}", area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let traiangle = Triangle {
            width: 10,
            height: 10,
        };
        let area = traiangle.calculate_area();

        println!("{area}");
    }

    #[test]
    fn works_2() {
        let rectangle = Rectangle {
            width: 10,
            height: 10,
        };
        let area = rectangle.calculate_area();

        println!("{area}");
    }
}
