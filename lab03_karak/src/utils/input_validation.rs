use derive_more::derive::Display;
use gtin_validate::gtin13;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zxcvbn::{zxcvbn, Score};

/// This function checks if the given password is valid
/// Returns true if the password is strong enough, false otherwise
fn password_validation(password: &str, username: &str) -> bool {
    // longueurs de pwd recommandées par l'OWASP cheat sheet
    if password.len() < 8 || password.len() > 256 {
        false
    }else{
        // Vérifie avec user_input si le username est dans le pwd
        // Un score de 3 est recommandé comme vérification
        zxcvbn(password, &[username]).score() >= Score::Three
    }
}

/// Interactively prompts the user for a password
pub fn password_input_validation(username: &str) -> String {
    loop {
        let password = inquire::Password::new("Enter your password :")
            .with_help_message("Password must be at least 8 and at most 256 characters long and be strong enough")
            .prompt()
            .expect("Unexpected failure in password input");

        if password_validation(&password, username) {
            return password;
        }

        println!("Password is too weak. Please try again...");

        let estimate = zxcvbn(&password, &[username]);
        if estimate.score() < Score::Three {
            println!("Password score was : {}, need at least 3 to be accepted.", estimate.score());
            if let Some(feedback) = estimate.feedback() {
                if let Some(warning) = feedback.warning() {
                    println!("WARNING : {}", warning);
                }
                println!("Suggestions :");
                for suggestion in feedback.suggestions() {
                    println!("- {}", suggestion);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Display, Error)]
pub struct InvalidInput;

/// Wrapper type for a username thas has been validated
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct Username(String);

impl TryFrom<String> for Username {
    type Error = InvalidInput;

    fn try_from(username: String) -> Result<Self, Self::Error> {
        username_validation(&username)?;
        Ok(Self(username))
    }
}

impl TryFrom<&str> for Username {
    type Error = InvalidInput;

    fn try_from(username: &str) -> Result<Self, Self::Error> {
        username_validation(username)?;
        Ok(Self(username.to_owned()))
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn username_validation(username: &str) -> Result<(), InvalidInput> {
    //On utilise un regex trouvé sur regexlib :0
    let regex_usern = Regex::new(r"^\w[a-zA-Z0-9_-]{2,28}\w$").unwrap();
    if !regex_usern.is_match(username) {
        Err(InvalidInput)
    }else {
        Ok(())
    }
}

pub fn username_input_validation(message: &str) -> Result<Username, InvalidInput> {
    let username = inquire::Text::new(message)
        .with_help_message(
            "Username must be at least 3 and at most 30 characters long and contain only alphanumeric characters, \"_\" or \"-\""
        )
        .prompt()
        .expect("Unexpected failure in username input");

    Username::try_from(username)
}

/// Wrapper type for an AVS number that has been validated
#[derive(Debug, Display, Serialize, Deserialize, Hash)]
pub struct AVSNumber(String);

impl TryFrom<String> for AVSNumber {
    type Error = InvalidInput;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if validate_avs_number(&value) {
            Ok(AVSNumber(value))
        } else {
            Err(InvalidInput)
        }
    }
}

fn validate_avs_number(avs_number: &str) -> bool {
    let regex_avs = Regex::new(r"^756\.?\d{4}\.?\d{4}\.?\d{2}$").unwrap();
    if !regex_avs.is_match(avs_number) {
        false
    } else {
        gtin13::check(&avs_number.replace('.', ""))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_validation() {
        let username = "user123";

        // Mot de passe valide
        assert!(password_validation("C@ts-and-Dogs-Living-together", username));

        // Trop court
        assert!(!password_validation("short", username));

        // Trop long
        assert!(!password_validation(&"a".repeat(257), username));

        // Contient le nom d'utilisateur
        assert!(!password_validation("user123password", username));

        // Faible score
        assert!(!password_validation("password123", username));
    }

    #[test]
    fn test_username_validation() {
        // Nom d'utilisateur valide
        assert!(username_validation("valid_username_1").is_ok());

        // Trop court
        assert!(username_validation("ab").is_err());

        // Trop long
        assert!(username_validation(&"a".repeat(31)).is_err());

        // Caractères invalides
        assert!(username_validation("invalid!username").is_err());

        // Caractères valides en bordure
        assert!(username_validation("a.username-1_b").is_ok());
    }

    #[test]
    fn test_avs_number_validation() {
        // Numéro AVS valide
        assert!(validate_avs_number("756.1234.5678.97"));

        // Format incorrect
        assert!(!validate_avs_number("123.4567.89"));

        // Numéro invalide avec checksum incorrecte
        assert!(!validate_avs_number("756.1234.5678.96"));

        // Numéro avec points omis
        assert!(validate_avs_number("7561234567897"));
    }

    #[test]
    fn test_username_try_from() {
        // Conversion réussie
        assert!(Username::try_from("valid_username".to_string()).is_ok());

        // Échec de conversion
        assert!(Username::try_from("invalid username!".to_string()).is_err());
    }

    #[test]
    fn test_avs_number_try_from() {
        // Conversion réussie
        assert!(AVSNumber::try_from("756.1234.5678.97".to_string()).is_ok());

        // Échec de conversion
        assert!(AVSNumber::try_from("123.4567.89".to_string()).is_err());
    }
}
