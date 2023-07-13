#[cfg(test)]
mod tests {

    #[test]
    fn work_0() {
        let mut x: i32 = 5;
        println!("The value is x is {x}");
        x = 6;
        println!("The value is x is {x}");
    }

    #[test]
    fn work_1() {
      let x: i32 = 5;

      let x: i32 = x + 1;

      {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
      }

      println!("The value of x is: {x}");
    }

    #[test]
    fn work_2() {
        let spaces: &str = "   ";
        let spaces: usize = spaces.len();

        println!("spaces: {spaces}");
    }

    #[test]
    fn work_3() {
        let c0: char = 'A';
        let c1: char = 'B';
        let c2: char = 'C';

        println!("{c0} {c1} {c2}");

        let ch_array: [char; 3] = ['A', 'B', 'C'];

        println!("{:?}", ch_array);

        let ch_str: &str = "ABC";

        println!("{:?}", ch_str);
    }

    #[test]
    fn work_4() {
        let c0: char = '러';
        let c1: char = '스';
        let c2: char = '트';

        println!("{c0} {c1} {c2}");

        let ch_array: [char; 3] = ['러', '스', '트'];

        println!("{:?}", ch_array);

        let ch_str: &str = "러스트";

        println!("{:?}", ch_str);
    }

    #[test]
    fn work_5() {
        let c0: char = '리';
        let c1: char = '콜';
        let c2: char = '라';

        println!("{c0} {c1} {c2}");

        let ch_array: [char; 3] = ['리', '콜', '라'];

        println!("{:?}", ch_array);

        let ch_str: &str = "리콜라";

        println!("{:?}", ch_str);
    }

    #[test]
    fn work_6() {
        let ss: String = String::from("ABCDEFGHIJK");

        for s in ss.chars() {
            println!("{s}");
        }

        for p in ss.as_bytes() {
            println!("{p}");
        }
    }

    #[test]
    fn work_7() {
        let string_literal: &str = "ABCDEFGHIJK";

        let sl: &str = string_literal;

        println!("{}", sl);
        println!("{}", string_literal);
    }

    #[test]
    fn work_8() {
        let array: [i32; 5] = [1, 2, 3, 4, 5];

        let another: [i32; 5] = array;

        println!("{:?}", array);
        println!("{:?}", another);
        println!("test");
    }

    #[test]
    fn work_9() {
        let array: [&str; 5] = ["a", "b", "c", "d", "e"];

        let another: [&str; 5] = array;

        let other: &[&str; 5] = &array;

        println!("{:?}", array);
        println!("{:?}", another);
        println!("{:?}", other);
        println!("test");
    }
}

