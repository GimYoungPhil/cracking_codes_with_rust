#[derive(Debug)]
enum Whisky {
    Blended(String),
    SingleMalt(String),
    Bourbone(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let whisky_0 = Whisky::Blended(String::from("Chivas Regal"));
        let whisky_1 = Whisky::SingleMalt(String::from("Glenfiddich"));
        let whisky_2 = Whisky::SingleMalt(String::from("Wild Turkey"));

        println!("{:?}", whisky_0);
        println!("{:?}", whisky_1);
        println!("{:?}", whisky_2);
    }

}
