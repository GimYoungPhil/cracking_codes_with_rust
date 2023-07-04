#[cfg(test)]
mod tests {

    #[test]
    fn work_0() {
        let mut x = 5;
        println!("The value is x is {x}");
        x = 6;
        println!("The value is x is {x}");
    }

    #[test]
    fn work_1() {
      let x = 5;

      let x = x + 1;

      {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
      }

      println!("The value of x is: {x}");
    }

    #[test]
    fn work_2() {
        let spaces = "   ";
        let spaces = spaces.len();

        println!("spaces: {spaces}");
    }

    #[test]
    fn work_3() {
        let a = 'a';
        let b = 'A';
        let c = '러';
        let d = '스';
        let e = '트';
        
        println!("a: {a}, b: {b} {c} {d} {e}");
    }
}

