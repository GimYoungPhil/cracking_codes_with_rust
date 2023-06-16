pub mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
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
    use super::gui::*;

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
                Box::new(button),
                Box::new(select_box),
            ],
        };

        screen.run();
    }
}
