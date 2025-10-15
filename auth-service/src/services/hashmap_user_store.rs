use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        let email = user.email.clone();
        if self.users.contains_key(&email) {
            Result::Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(email, user);
            Ok(())
        }
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            Ok(user.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }


    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let new_user = User::new("test_email".to_string(), "test_password".to_string(), true);
        let result_first_add = store.add_user(new_user.clone());
        assert_eq!(Ok(()), result_first_add);
        let result_second_add = store.add_user(new_user);
        assert_eq!(Err(UserStoreError::UserAlreadyExists), result_second_add);
    }

    #[tokio::test]
    async fn test_get_user() {
        let a_user = User::new("one".to_string(), "two".to_string(), true); 
        let store = HashmapUserStore {
            users: HashMap::from([
                ("one".to_string(), a_user.clone()),
            ])
        };
        let known_user = store.get_user("one").expect("The user should be known");
        assert_eq!(known_user, a_user);
        let unknown_user = store.get_user("ten").err().expect("The user should not be known");
        assert_eq!(unknown_user, UserStoreError::UserNotFound);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let a_user = User::new("one".to_string(), "two".to_string(), true); 
        let store = HashmapUserStore {
            users: HashMap::from([
                ("one".to_string(), a_user.clone()),
            ])
        };
        let valid_user = store.validate_user("one", "two");
        assert_eq!(valid_user, Ok(()));
        let invalid_user = store.validate_user("one", "three");
        assert_eq!(invalid_user, Err(UserStoreError::InvalidCredentials));
        let unknown_user = store.validate_user("ten", "eleven").err().expect("The user should not be known");
        assert_eq!(unknown_user, UserStoreError::UserNotFound);    
    }
}
