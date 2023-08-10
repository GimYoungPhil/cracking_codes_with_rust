mod function_with_i32 {
    pub fn method_0(v: &i32) {
        println!("{v}");
    }
    pub fn method_1(v: &mut i32) {
        *v = 100;
        println!("{v}");
    }
    pub fn method_2(v: i32) {
        println!("{v}");
    }
    pub fn method_3(mut v: i32) {
        v = 100;
        println!("{v}");
    }
}

mod function_with_char {
    pub fn method_0(v: &char) {
        println!("{v}");
    }
    pub fn method_1(v: &mut char) {
        *v = 'z';
        println!("{v}");
    }
    pub fn method_2(v: char) {
        println!("{v}");
    }
    pub fn method_3(mut v: char) {
        v = 'z';
        println!("{v}");
    }
}

mod function_with_string {
    pub fn method_0(v: &String) {
        println!("{v}");
    }
    pub fn method_1(v: &mut String) {
        v.push('!');
        println!("{v}");
    }
    pub fn method_2(v: String) {
        println!("{v}");
    }
    pub fn method_3(mut v: String) {
        v.push('!');
        println!("{v}");
    }
}

mod function_with_str {
    pub fn method_0(v: &str) {
        println!("{v}");
    }
    pub fn method_1(v: &mut str) {
        // v.push('!');
        println!("{v}");
    }
    // pub fn method_2(v: str) {
    //     println!("{v}");
    // }
    // pub fn method_3(mut v: str) {
    //     v.push('!');
    //     println!("{v}");
    // }
}

mod function_with_vec {
    pub fn method_0(v: &Vec<i32>) {
        println!("{:?}", v);
    }
    pub fn method_1(v: &mut Vec<i32>) {
        v.push(100);
        println!("{:?}", v);
    }
    pub fn method_2(v: Vec<i32>) {
        println!("{:?}", v);
    }
    pub fn method_3(mut v: Vec<i32>) {
        v.push(100);
        println!("{:?}", v);
    }
}

mod function_with_array {
    pub fn method_0(v: &[i32; 3]) {
        println!("{:?}", v);
    }
    pub fn method_1(v: &mut [i32; 3]) {
        // v.push(100);
        println!("{:?}", v);
    }
    pub fn method_2(v: [i32; 3]) {
        println!("{:?}", v);
    }
    pub fn method_3(mut v: [i32; 3]) {
        // v.push(100);
        println!("{:?}", v);
    }
}
mod function_with_slice {
    pub fn method_0(v: &[i32]) {
        println!("{:?}", v);
    }
    pub fn method_1(v: &mut [i32]) {
        // v.push(100);
        println!("{:?}", v);
    }
    // pub fn method_2(v: [i32]) {
    //     println!("{:?}", v);
    // }
    // pub fn method_3(mut v: [i32]) {
    //     v.push(100);
    //     println!("{:?}", v);
    // }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_1() {
        let mut list = [1, 2, 3];
        function_with_slice::method_1(&mut list[..]);
    }
}
