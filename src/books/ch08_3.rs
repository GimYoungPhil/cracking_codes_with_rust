use std::{collections::HashMap, hash::Hash};

fn ex_20() {
    let mut scores  = HashMap::new();

    scores.insert(String::from("Red"), 1);
    scores.insert(String::from("Blue"), 2);
    scores.insert(String::from("Green"), 3);

    println!("{:?}", scores );
}

fn ex_21() {
    let mut scores  = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Green"), 30);

    let key = String::from("Green");
    let value = scores.get(&key).copied().unwrap();

    println!("{:?}", value);
}

fn ex_22() {
    let mut scores  = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Green"), 30);

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

fn ex_23() {
    let mut scores  = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Green"), 30);

    scores.insert(String::from("Green"), 2500);

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

fn ex_24() {
    let mut scores  = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Green"), 30);
    
    scores.entry(String::from("Blue")).or_insert(200);
    scores.entry(String::from("Green")).or_insert(300);

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

fn ex_25() {
    let mut scores  = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Green"), 30);
    
    scores.entry(String::from("Blue")).or_insert(200);
    *scores.entry(String::from("Green")).or_insert(300) *= 2;

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

fn ex_26() {
    let text = "hello world wonderful world world war under world mario world";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }

    for (key, value) in map {
        println!("{key}: {value}");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_0() {
        ex_26();
    }

}
