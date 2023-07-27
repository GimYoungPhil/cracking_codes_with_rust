fn call_i32(v: i32) {
    println!("value: {}", v);
}

fn call_i32_mut(mut v: i32) {
    println!("value: {}", v);

    v = 20;
    println!("value: {}", v);
}

fn call_r32(v: &i32) {
    println!("value: {}", v);
}

fn call_r32_mut(v: &mut i32) {
    println!("value: {}", v);

    *v = 20;
    println!("value: {}", v);
}

fn lifetime_i32<'a>(v: &'a i32) -> &'a i32 {
    println!("value: {}", v);

    v
}

fn lifetime_i32_mut<'a>(v: &'a mut i32) -> &'a mut i32 {
    println!("value: {}", v);

    *v = 20;

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let num = 10;

        call_i32(num);

        println!("done: {}", num);
    }

    #[test]
    fn works_2() {
        let num = 10;

        call_i32_mut(num);

        println!("done: {}", num);
    }

    #[test]
    fn works_3() {
        let num = 10;

        call_r32(&num);

        println!("done: {}", num);
    }

    #[test]
    fn works_4() {
        let mut num = 10;

        call_r32_mut(&mut num);

        println!("done: {}", num);
    }

    #[test]
    fn works_5() {
        let num = 10;

        lifetime_i32(&num);

        println!("done: {}", num);
    }

    #[test]
    fn works_6() {
        let mut num = 10;

        let re = lifetime_i32_mut(&mut num);

        println!("done: {}", re);

        *re = 30;

        println!("done: {}", num);
    }
}
