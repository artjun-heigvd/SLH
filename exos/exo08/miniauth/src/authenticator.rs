use std::cmp::Ordering;
use thiserror::Error;
use totp_rs::{Secret, TOTP};


/// A string that has been validated to be a valid username
#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Username(String);

impl Username {
    /// Check if a string is a valid username
    /// 
    /// ```
    /// assert!(Username::try_from("admin".to_owned()).is_some())
    /// assert!(Username::try_from("+\"*ç%&/()".to_owned()).is_none())
    /// ``` 
    fn try_from(username: String) -> Option<Self> {
        /// Todo: validate username
        Some(Username(username))
    }
}

/// Cheap conversion to string slice
impl AsRef<str> for Username {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Debug,Clone)]
pub struct PwHash(String);

impl PwHash {

    /// Generate a dummy password hash. Why do we need this ?
    fn default() -> Self {
        todo!()
    }

    /// Generate a hash
    fn generate(username: &Username, password: &str) -> Self {
        todo!()
    }

    fn validate(&self, password: &str) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct User {
    username: Username,
    totp: TOTP,
    password: String,
}


/// Errors that may occur when creating a new user
#[derive(Debug,Error)]
pub enum RegisterError {
    #[error("This is not a valid username")]
    InvalidUsername
}

impl User {
    /// Create a new user
    /// 
    /// The password is hashed with Argon2id and stored in the user data
    /// A new random OTP key is generated and stored in the user data
    /// 
    /// ```
    /// let user = User::register("admin".to_owned(), "hunter2").unwrap();
    /// ```
    pub fn register(username: String, password: &str) -> Result<User, RegisterError> {
        todo!()
    }

    /// Get the username of the current user
    /// 
    /// ```
    /// let user = User::register("admin".to_owned(), "hunter2").unwrap();
    /// assert_eq!(user.username(), "admin");
    /// ```
    pub fn username(&self) -> &str {
        &self.username.0
    }

    /// Retrieve the secret OTP url for this user.
    /// 
    /// The URL can be loaded into the authenticator app with a QR code
    pub fn get_totp_url(&self) -> String {
        todo!()
    }

    /// Tests if the password is correct
    /// 
    /// 
    pub fn authenticate_password(&self, password: &str) -> bool {
        todo!()
    }

    /// Tests if the OTP code is correct
    pub fn authenticate_otp(&self, code: &str) -> bool {
        todo!()
    }
    
}