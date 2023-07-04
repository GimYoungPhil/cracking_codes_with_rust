#[cfg(test)]
mod tests {

    #[test]
    fn ex11() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }

    #[test]
    fn ex12() {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;

        println!("x: {x}, y: {y}, x: {z}");
    }

    #[test]
    fn ex13() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let five_handred = tup.0;

        let siz_point_four =  tup.1;

        let one = tup.2;
    }

    #[test]
    fn ex14() {
        let a = [1, 2, 3, 4, 5];

        let months = ["Jenuary", "February", "March", "April", "May", "June", "July",
                                  "August", "September", "October", "November", "December"];

        let a: [i32; 5] = [1, 2, 3, 4, 5];

        let a = [3; 5];
    }

    #[test]
    fn ex15() {
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];

        println!("first: {first}");
        println!("second: {second}");
        println!("a: {:?}", a);
    }

    #[test]
    #[should_panic]
    fn ex16() {
        let a = [1, 2, 3, 4, 5];

        let element = a[10];

        println!("element: {element}");
    }
}





