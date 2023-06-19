pub mod chapters {
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
    mod state_behavior;
}

pub mod etc {
    mod some_trait;
    mod associated_type;
    mod generic;
    mod generic2;
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}
