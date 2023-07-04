pub mod chapters {
    pub mod chapter_00;
    pub mod chapter_02;
    // pub mod chapter_02_c;
}

pub mod lang {
    // mod exercise;
    // mod iterator;
    // mod pointers;
    // mod oop;
    // mod trait_object;
    // mod trait_generic;
    // mod state_pattern;
    // mod state_behavior;
    // mod closer;
    // mod closer2;
}

pub mod etc {
    // mod some_trait;
    // mod associated_type;
    // mod generic;
    // mod generic2;
}

pub mod docs {
    // mod ch00;
    // mod ch05;
    // mod ch13;
}

pub mod books {
    // mod ch03;
    // mod ch03_2;
    pub mod ch04;
    pub mod ch04_3;
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}
