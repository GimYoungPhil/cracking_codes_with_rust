struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_0() {
        let user = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        let email = String::from("someone@example.com");
        assert_eq!(user.email, email);
    }

    #[test]
    fn work_1() {
        let mut user = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        user.email = String::from("anotheremail@example.com");

        let email = String::from("anotheremail@example.com");
        assert_eq!(user.email, email);
    }

    #[test]
    fn work_2() {
        let mut user = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        let count = &mut user.sign_in_count;
        *count += 2;

        assert_eq!(user.sign_in_count, 3);
    }
}
