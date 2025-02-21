//! Hachage et vérification des mots de passe

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHashString, PasswordVerifier, SaltString},
    Argon2, PasswordHasher,
};
use derive_more::derive::Display;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::LazyLock};

static DEFAULT_HASHER: LazyLock<Argon2<'static>> = LazyLock::new(|| Argon2::default());

/// Le hash d'un mot de passe vide, à utiliser quand l'utilisateur n'existe pas
/// pour éviter une attaque par canal auxiliaire
static EMPTY_HASH: LazyLock<PWHash> = LazyLock::new(|| hash(""));

/// Un mot de passe haché
#[derive(Clone, Debug, Display)]
pub struct PWHash(PasswordHashString);

impl std::hash::Hash for PWHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state)
    }
}

impl Serialize for PWHash {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PWHash {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let hash = PasswordHashString::from_str(&s)
            .map_err(|_| <D::Error as serde::de::Error>::custom("Invalid PHC string"))?;
        Ok(PWHash(hash))
    }
}

/// Calcule un haché a partir d'un mot de passe en clair, en choisissant un sel au hasard
pub fn hash(password: &str) -> PWHash {
    let salt = SaltString::generate(&mut OsRng);
    let hash = DEFAULT_HASHER
        .hash_password(password.as_bytes(), &salt)
        .expect("Password hashing failed unexpectedly")
        .serialize();
    PWHash(hash)
}

/// Vérifie si le mot de passe correspond au hash stocké.
/// 
/// Si un hash n'est pas fourni, on doit quand même tester
/// le mot de passe avec un faux hash pour éviter une timing
/// attack.
pub fn verify(password: &str, maybe_hash: Option<&PWHash>) -> bool {
    let hash = maybe_hash.unwrap_or(&*EMPTY_HASH);

    DEFAULT_HASHER.verify_password(password.as_bytes(), &hash.0.password_hash()).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "StrongP@ssw0rd";

        // Hashage du mot de passe
        let hashed = hash(password);

        // Vérification du mot de passe valide
        assert!(verify(password, Some(&hashed)));

        // Vérification avec un mot de passe incorrect
        assert!(!verify("WrongPassword", Some(&hashed)));
    }

    #[test]
    fn test_verify_empty_password() {
        // Test avec un mot de passe vide
        let hashed_empty = hash("");

        assert!(verify("", Some(&hashed_empty)));
        assert!(!verify("NotEmpty", Some(&hashed_empty)));
    }

    #[test]
    fn test_verify_with_empty_hash() {
        let password = "SomePassword";

        // Test avec un hash vide (attaque par canal auxiliaire simulée)
        assert!(!verify(password, None));

        // Toujours faux avec un mot de passe différent
        assert!(!verify("DifferentPassword", None));
    }

    #[test]
    fn test_hash_uniqueness() {
        let password = "UniquePassword";

        // Deux hachés du même mot de passe ne doivent pas être identiques (sel aléatoire)
        let hash1 = hash(password);
        let hash2 = hash(password);
        assert_ne!(hash1.to_string(), hash2.to_string());
    }

    #[test]
    fn test_hash_serialization() {
        let password = "SerializablePassword";

        // Sérialisation et désérialisation du haché
        let hashed = hash(password);
        let serialized = serde_json::to_string(&hashed).expect("Serialization failed");
        let deserialized: PWHash = serde_json::from_str(&serialized).expect("Deserialization failed");

        // Vérification que la désérialisation produit le même haché
        assert_eq!(hashed.to_string(), deserialized.to_string());
    }
}
