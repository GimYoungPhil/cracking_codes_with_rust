use std::collections::HashMap;

pub fn median(list: &[i32]) -> &i32 {
    let half = list.len() / 2;

    let median = &list[half];

    median
}

pub fn mode(list: &[i32]) -> &i32 {
    let mut map = HashMap::new();

    for num in list {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut mode = &list[0];
    let mut frequency = map.get(mode).copied().unwrap_or(0);

    for (key, value) in map {
        if frequency < value {
            frequency = value;
            mode = key;
        }
    }

    mode
}

fn ex_1(list: &[i32]) -> (&i32, &i32) {
    let median = median(list);

    let mode = mode(list);

    println!("median: {median}, mode: {mode}");

    (median, mode)
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
        let list = [1, 2, 3, 4, 5, 4, 1, 2, 3, 3, 2];
        assert_eq!(ex_1(&list), (&4, &3));
    }
}
