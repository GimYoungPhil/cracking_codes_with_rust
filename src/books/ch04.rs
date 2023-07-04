pub fn mm() {
    const SSS: &str = "cool";

    let v = [10; SSS.len()];
    println!("{:?}", v);
}

pub fn pp() {
    let mut s = String::from("hello");

    let r3 = &mut s;
    println!("{}", r3);

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
}

pub fn oo() {
    let s: String = String::from("hello");

    println!("{:?}", s);

    let bytes: &[u8] = s.as_bytes();

    println!("{:?}", bytes);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_0() {
        let b: bool = true;
        let v: &[i32];
        if b {
            v = &[1, 2, 3, 4, 5];
        } else {
            v = &[10, 20];
        }

        println!("{:?}", v);
    }

    #[test]
    fn work_1() {
        oo();
    }
  }
