// The User struct should contain 3 fields. email, which is a String; 
// password, which is also a String; and requires_2fa, which is a boolean. 
#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: String,
    pub password: String,
    requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> User {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}
