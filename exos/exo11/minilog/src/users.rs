use std::collections::BTreeMap;
use anyhow::{anyhow, Result};
use tracing::warn;
use validator::ValidateEmail;

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd)]
pub struct Email(String);

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Default)]
pub struct DB {
    inner: BTreeMap<Email,String>
}

// TODO log validation errors
fn email_from_string(email: String) -> Result<Email> {
    if email.validate_email() {
        Ok(Email(email))
    } else {
        Err(anyhow!("Validation error"))
    }

}



impl DB {
    pub fn register(&mut self, email: String, password: String) -> Result<()> {
        // TODO log register failures and successes
        let email = email_from_string(email)?;
        self.inner.insert(email, password);
        Ok(())
    }

    pub fn login(&self, email: String, password: &str) -> Option<Email> {
        // TODO log login successes and failures

        let email = email_from_string(email).ok()?;

        //TODO: implement password hashing and make this timing-attack resistant
        let p = self.inner.get(&email)?;

        if password != p { return None }

        Some(email)

    }
}