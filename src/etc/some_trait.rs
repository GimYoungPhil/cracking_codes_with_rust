struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn eat(&self) -> String {
        format!("eat chocolate({}) bars", self.age / 2)
    }
}

trait Exercise {
    fn run(&self) -> String;
}

impl Exercise for Person {
    fn run(&self) -> String {
        format!("run {} minutes", self.age - 10)
    }
}

fn marathon<T: Exercise>(t: T) {
    let mut list: Vec<T> = vec![];

    list.push(t);

    for t in &list {
        println!("{}", t.run());
    }
}

struct Monster {
    name: String,
    age: u8,
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let name = String::from("Tom");
        let tom = Person { name, age: 42 };

        assert_eq!(42, tom.age);
        assert_eq!("eat chocolate(21) bars", tom.eat());
        assert_eq!("running 32 minutes", tom.run());
    }

    #[test]
    fn it_works_2() {
        let name = String::from("Tom");
        let tom = Person { name, age: 42 };

        marathon(tom);
    }

    #[test]
    fn it_works_3() {
        let name = String::from("T-Rex");
        let mon = Monster { name, age: 142 };
        
        // marathon(mon);
    }
}
