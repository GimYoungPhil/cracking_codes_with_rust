pub mod basic {
    use std::collections::HashMap;

    pub fn sum(collection: &[i32]) -> i32 {
        let mut sum = 0;
        for i in collection {
            sum += *i;
        }
        sum
    }

    pub fn average(collection: &[i32]) -> i32 {
        let sum= sum(collection);
        let length = collection.len();
        sum / (length as i32)
    }

    pub fn favorite(collection: &[i32]) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in collection {
            let count = map.entry(*num).or_insert(0);
            *count += 1;
        }

        let mut max_key = 0;
        let mut max_value = 0;

        for (key, value) in &map {
            if max_value < *value {
                max_key = *key;
                max_value = *value;
            }
        }

        max_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        let quest = vec![10, 20, 30, 40, 50];
        let result = basic::sum(&quest[..]);
        assert_eq!(result, 150);
    }

    #[test]
    fn it_works_1() {
        let q0 = vec![11, 20, 30, 40, 50];
        let result = basic::average(&q0[..]);
        assert_eq!(result, 30);
    }

    #[test]
    fn it_works_2() {
        let q0 = vec![10, 20, 30, 40, 50, 10, 20, 30, 10];
        let result = basic::favorite(&q0[..]);
        assert_eq!(result, 10);
    }
}
