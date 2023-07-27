trait LifetimeElisionRules {
    // 1. input lifetime
    fn foo_a<'a>(x: &'a i32);

    fn foo_b<'a, 'b>(x: &'a i32, y: &'a i32);

    // 2. output lifetime
    fn foo_c<'a>(x: &'a i32) ->&'a i32;

    // 3. output lifetime
    fn foo_d<'a>(&'a self, x: &i32) -> &'a i32;

    fn foo_e<'a>(&'a mut self, x: &i32) -> &'a i32;
}

fn foo_a(x: &i32) {
    println!("[foo_a] x: {}", x);
}

fn foo_b(x: &i32, y: &i32) {
    println!("[foo_b] x: {}, y: {}", x, y);
}

fn foo_c(x: &i32) -> &i32 {
    println!("[foo_c] x: {}", x);
    x
}

struct Coffee {
    volume: i32,
}

impl Coffee {
    fn foo_d(&self, x: &i32) -> &i32 {
        &self.volume
    }

    fn foo_e(&mut self, x: &i32) -> &i32 {
        self.volume += x;

        &self.volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let x = 10;
        let y = 20;

        foo_a(&x);
        foo_b(&x, &y);
        let re = foo_c(&x);

        println!("{}", re);
    }

    #[test]
    fn works_2() {
        let coff = Coffee {
            volume: 330,
        };
        let x = 10;
        let vol = coff.foo_d(&x);

        println!("{}", vol);
    }

    #[test]
    fn works_3() {
        let mut coff = Coffee {
            volume: 330,
        };
        let x = 10;
        let vol = coff.foo_e(&x);

        println!("{}", vol);
    }
}
