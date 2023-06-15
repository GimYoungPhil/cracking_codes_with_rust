pub mod chapters {
    // verify_remainder
    pub mod chapter_00;

    //
    // mod chapter_01;

    // caesar
    pub mod chapter_02;
}

pub mod lang {
    pub mod exercise;
    pub mod iterator;
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}
