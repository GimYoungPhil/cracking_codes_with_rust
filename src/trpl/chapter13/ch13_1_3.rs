
fn consume_with_relish<F>(func: F)
    where F: FnOnce() -> String
{
    println!("Consumed: {}", func());

    println!("Delicious!");
}

fn do_twice<F>(mut func: F)
    where F: FnMut()
{
    func();
    func();
}

fn call_with_one<F>(func: F) -> usize
    where F: Fn(usize) -> usize
{
    func(1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        // 캡처된 값을 클로저 밖으로 이동시키기
        let x = String::from("x");
        let consume_and_return_x = move || x;
        consume_with_relish(consume_and_return_x);
    }

    #[test]
    fn works_2() {
        // 캡처된 값을 변형시키기
        let mut x: usize = 1;
        {
            let add_two_to_x = || x += 2;
            do_twice(add_two_to_x);
        }
        assert_eq!(x, 5);
    }

    #[test]
    fn works_3() {
        // 캡처하기 않기
        let double = |x| x * 2;
        assert_eq!(call_with_one(double), 2);
    }
}
