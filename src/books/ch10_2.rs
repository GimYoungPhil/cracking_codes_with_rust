struct Triangle {
    width: i32,
    height: i32,
}

impl Triangle {
    fn area(&self) -> f32 {
        let result = (self.width as f32) * (self.height as f32) * 0.5f32;
        result
    }
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let result = (self.width as f32) * (self.height as f32);
        result
    }
}

// fn display<T>(list: &[T]) {
//     for item in list {
//         let area = item.area();
//         println!("Area: {}", area);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let traiangle = Triangle {
            width: 10,
            height: 10,
        };
        let area = traiangle.area();

        println!("{area}");
    }

    #[test]
    fn works_2() {
        let rectangle = Rectangle {
            width: 10,
            height: 10,
        };
        let area = rectangle.area();

        println!("{area}");
    }
}
