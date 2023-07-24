mod ex1 {
    struct Point<T> {
        x: T,
        y: T,
    }

    
    fn ex_01() {
        let integer = Point { x: 5, y: 10 };

        let float = Point { x: 1.0, y: 4.8 };
    }
}

mod ex2 {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    fn ex_02() {
        let both_integer = Point { x: 5, y: 10 };

        let both_float = Point { x: 1.0, y: 4.0 };

        let integer_and_float = Point { x: 5, y: 4.0 };
    }
}

pub mod ex3 {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    pub fn run() {
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }
}

pub mod ex4 {
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    pub fn run() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };
    
        let p3 = p1.mixup(p2);
    
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        ex3::run();
    }

    #[test]
    fn works_2() {
        ex4::run();
    }
}
