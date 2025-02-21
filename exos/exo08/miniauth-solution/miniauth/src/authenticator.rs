use std::sync::LazyLock;
use argon2::{password_hash::{PasswordHashString, SaltString}, Argon2, Params, PasswordHasher, PasswordVerifier};
use rand_core::{OsRng, RngCore};
use thiserror::Error;
use totp_rs::TOTP;


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
        // Todo: validate username
        Some(Username(username))
    }
}

/// Cheap conversion to string slice
impl AsRef<str> for Username {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Debug,Clone)]
pub struct PwHash(PasswordHashString);

const PEPPER: &[u8] = b"alkdsjaa019283091283lksdjgoiqwedoijasoidjaoisjdoaid";
const APP_ID: &str = "MINIAUTH-1.0";

const ARGON: LazyLock<Argon2<'static>> = LazyLock::new(|| {
    Argon2::new_with_secret(PEPPER,argon2::Algorithm::Argon2id, argon2::Version::V0x13, Params::default()).unwrap()
});

impl PwHash {

    /// Generate a dummy password hash. Why do we need this ?
    fn default() -> Self {
        Self::generate("dummy")
    }

    /// Generate a hash
    fn generate(password: &str) -> Self {
        let rng = &mut argon2::password_hash::rand_core::OsRng;
        let salt = SaltString::generate(rng);
        let hash = ARGON
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .serialize();

        Self(hash)
    }

    fn validate(&self, password: &str) -> bool {
        ARGON.verify_password(password.as_bytes(), &self.0.password_hash()).is_ok()
    }
}

#[derive(Debug)]
pub struct User {
    username: Username,
    totp: TOTP,
    password: PwHash,
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
        let username = Username::try_from(username).ok_or(RegisterError::InvalidUsername)?;
        let password = PwHash::generate(password);

        let mut secret = vec![0u8; 32];
        OsRng::fill_bytes(&mut OsRng, &mut secret[..]);

        let totp = TOTP::new(totp_rs::Algorithm::SHA256, 6, 1, 60, secret,
             Some(APP_ID.to_owned()), username.as_ref().to_owned())
             .unwrap();
        Ok(User { username, password, totp, })
    }

    pub fn default() -> Self {
        User { username: Username("".to_owned()), totp: TOTP::new(totp_rs::Algorithm::SHA256, 6, 1, 60, vec![], None, "".to_owned()).unwrap(), password: PwHash::default() }
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
        self.totp.get_url()
    }

    /// Tests if the password is correct
    /// 
    /// 
    pub fn authenticate_password(&self, password: &str) -> bool {
        self.password.validate(password)
    }

    /// Tests if the OTP code is correct
    pub fn authenticate_otp(&self, code: &str) -> bool {
        self.totp.check_current(code).unwrap_or_default()
    }
    
}