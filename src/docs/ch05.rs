#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn add_width(&mut self, value: u32) {
        self.width += value;
    }
    fn remove(self) {
        println!("Rectangle Removed...");
    }
    fn dobule(self) -> Self {
        Self {
            width: self.width * 2,
            height: self.height * 2,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rect = Rectangle { width: 100, height: 200 };
        let area = rect.area();

        assert_eq!(area, 20000);
    }

    #[test]
    fn it_works_1() {
        let mut rect = Rectangle { width: 100, height: 200 };
        let area = rect.area();
        assert_eq!(area, 20000);

        rect.add_width(50);
        let area = rect.area();
        assert_eq!(area, 30000);
    }

    #[test]
    fn it_works_2() {
        let rect = Rectangle { width: 100, height: 200 };
        let area = rect.area();
        assert_eq!(area, 20000);

        rect.remove();
        // let area = rect.area();
        // assert_eq!(area, 20000);
    }

    #[test]
    fn it_works_3() {
        let mut rect = Rectangle { width: 100, height: 200 };
        rect.add_width(50);
        let doubled_rect = rect.dobule();

        let area = doubled_rect.area();
        assert_eq!(area, 120000);
    }
}
