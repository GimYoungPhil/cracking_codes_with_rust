pub mod guii {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T: Draw> Screen<T> {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Button {{ w: {}, h: {} }} was drawn.", self.width, self.height);
        }
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("SelectBox {{ w: {}, h: {} }} was drawn.", self.width, self.height);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::guii::*;

    #[test]
    fn it_works() {
        let select_box = SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        };

        let button = Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        };

        let screen = Screen {
            components: vec![
                button,
                // select_box,
            ],
        };

        screen.run();
    }
}
