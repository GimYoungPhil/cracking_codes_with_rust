pub trait Area {
    fn calculate_area(&self) -> f32;
}

struct Triangle {
    width: i32,
    height: i32,
}

impl Area for Triangle {
    fn calculate_area(&self) -> f32 {
        let result = (self.width as f32) * (self.height as f32) * 0.5f32;
        result
    }
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Area for Rectangle {
    fn calculate_area(&self) -> f32 {
        let result = (self.width as f32) * (self.height as f32);
        result
    }
}

fn display<T: Area>(list: &[T]) {
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
