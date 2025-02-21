use thiserror::Error;
use super::authenticator::User;

/// Used for database mocking
/// 
pub struct Database {
    inner: Vec<User>
}

#[derive(Debug,Error)]
pub enum StoreError {
    #[error("User already exists")]
    UserAlreadyExists
}

/// Used internally to keep the database sorted
fn user_compare(a: &User, b: &User) -> std::cmp::Ordering {
    a.username().cmp(b.username())
}

impl Database {
    /// Create a new empty in-memory user database
    /// 
    /// ```
    /// let mut db = Database::new();
    /// ```
    pub fn new() -> Self { 
        Self { inner: vec![] }
    }

    /// Add a user to the database
    /// 
    /// ```
    /// let mut db = Database::new();
    /// let user = User::register("admin", "hunter2").unwrap();
    /// db.store(user);
    /// ```
    pub fn store(&mut self, user: User) -> Result<(), StoreError> {
        if self.fetch(user.username()).is_some() {
            return Err(StoreError::UserAlreadyExists)
        }

        self.inner.push(user);
        self.inner.sort_unstable_by(user_compare);

        Ok(())
    }

    /// Fetch a user from the database
    /// ```
    /// let mut db = Database::new();
    /// let user = User::register("admin", "hunter2").unwrap();
    /// db.store(user).unwrap();
    /// 
    /// let user2 = db.fetch("admin").unwrap();
    /// assert_eq!(format!("{user:?}"), format!("{user2:?}"));
    /// 
    /// assert!(db.fetch("nobody").is_none())
    /// ```
    pub fn fetch(&self, username: &str) -> Option<&User> {
        let idx = self.inner.binary_search_by(|u| u.username().cmp(username)).ok()?;
        Some(&self.inner[idx])
    }
 }
 