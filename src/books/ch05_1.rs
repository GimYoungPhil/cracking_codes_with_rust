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

        assert_eq!(user.email, String::from("anotheremail@example.com"));
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

    #[test]
    fn work_3() {
        let user = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        let user2 = User {
            email: String::from("another@example.com"),
            ..user
        };

        assert_eq!(user.active, true);
        assert_eq!(user.email, String::from("someone@example.com"));
        assert_eq!(user2.sign_in_count, 1);
    }

    #[test]
    fn work_4() {
        let user = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        let active = user.active;
        let email = user.email;
        // let u = user;


    }
}
