pub mod chapters {
    // verify_remainder
    pub mod chapter_00;

    //
    // mod chapter_01;

    // caesar
    pub mod chapter_02;
}

pub mod lang {
    mod exercise;
    mod iterator;
    mod pointers;
    mod oop;
    mod trait_object;
    mod trait_generic;
    mod state_pattern;
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}
