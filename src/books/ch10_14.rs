fn call_no_life(v: &i32) -> &i32 {
    println!("value: {}", v);

    v
}

fn call_on_life<'a>(v: &'a i32) -> &'a i32 {
    println!("value: {}", v);

    v
}

fn call_no_life_mut(v: &mut i32) -> &i32 {
    println!("value: {}", v);

    *v = 20;

    v
}

fn call_on_life_mut<'a>(v: &'a mut i32) -> &'a i32 {
    println!("value: {}", v);

    *v = 20;

    v
}

fn first<'a>(v1: &'a i32, v2: &i32) -> &'a i32 {
    println!("first: {}", v1);
    println!("second: {}", v2);

    v1
}

fn second<'a>(v1: &i32, v2: &'a i32) -> &'a i32 {
    println!("first: {}", v1);
    println!("second: {}", v2);

    v2
}

fn either<'a>(v1: &'a i32, v2: &'a i32) -> &'a i32 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let result;
        {
            let num = 10;
            result = call_no_life(&num);
            println!("num: {}", num);
            println!("result: {}", result);
        }

    }

    #[test]
    fn works_2() {
        let result;
        {
            let num = 10;
            result = call_on_life(&num);
            println!("num: {}", num);
            println!("result: {}", result);
        }
    }

    #[test]
    fn works_3() {
        let result;
        {
            let mut num = 10;
            result = call_no_life_mut(&mut num);
            println!("result: {}", result);
            println!("num: {}", num);
        }
    }

    #[test]
    fn works_4() {
        let result;
        {
            let mut num = 10;
            result = call_on_life_mut(&mut num);
            println!("result: {}", result);
            println!("num: {}", num);
        }
    }

    #[test]
    fn works_5() {
        let result;
        let f = 10;
        {
            let s = 20;
            result = first(&f, &s);
        }
        println!("result: {}", result);
    }

    #[test]
    fn works_6() {
        let result;
        let s = 20;
        {
            let f = 10;
            result = second(&f, &s);
        }
        println!("result: {}", result);
    }

}
