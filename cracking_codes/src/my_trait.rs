pub trait Summariziable {
    fn summary(&self) -> String;
}

pub fn display_show<T: Summariziable>(obj: T) -> String {
    format!("DDS:: {}", &obj.summary())
}

pub struct NewsArticle {
    text: String,
}

pub struct Tweet {
    message: String,
}

impl Summariziable for NewsArticle {
    fn summary(&self) -> String {
        format!("NewsArticle: {}", &self.text)
    }
}

impl Summariziable for Tweet {
    fn summary(&self) -> String {
        format!("Tweet: {}", &self.message)
    }
}


pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let news = NewsArticle { text: String::from("Hello world") };
        assert_eq!(news.text, "Hello world".to_string());
    }

    #[test]
    fn it_works_1() {
        let tweet = Tweet { message: String::from("헬로우 월드") };
        assert_eq!(tweet.message, "헬로우 월드".to_string());
    }

    #[test]
    fn it_works_2() {
        let news = NewsArticle { text: String::from("Hello world") };
        assert_eq!(news.summary(), "NewsArticle: Hello world".to_string());
    }

    #[test]
    fn it_works_3() {
        let tweet = Tweet { message: String::from("헬로우 월드") };
        assert_eq!(tweet.summary(), "Tweet: 헬로우 월드".to_string());
    }

    #[test]
    fn it_works_4() {
        let news = NewsArticle { text: String::from("Hello world") };
        assert_eq!(display_show(news), "DDS:: NewsArticle: Hello world".to_string());
    }


    #[test]
    fn it_works_5() {
        let tweet = Tweet { message: String::from("헬로우 월드") };
        assert_eq!(display_show(tweet), "DDS:: Tweet: 헬로우 월드".to_string());
    }

    #[test]
    fn it_works_6() {
        let numbers = vec![34, 50, 25, 100, 65];
        assert_eq!(largest(&numbers), &100);
    }
}
