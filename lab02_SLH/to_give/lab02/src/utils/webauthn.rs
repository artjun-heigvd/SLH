//! Gère l'intégration de WebAuthn pour l'enregistrement, l'authentification, et la récupération.
//! Fournit des fonctions pour démarrer et compléter les processus d'enregistrement et d'authentification.
//! Inclut également des mécanismes pour la gestion sécurisée des passkeys et des tokens de récupération.

use std::collections::HashMap;
use anyhow::{Result, Context};
use webauthn_rs::prelude::*;
use once_cell::sync::Lazy;
use url::Url;
use tokio::sync::RwLock;
use crate::database;
use crate::database::user::{get_passkey, set_passkey};

// Initialisation globale de WebAuthn
static WEBAUTHN: Lazy<Webauthn> = Lazy::new(|| {
    let rp_id = "localhost";
    let rp_origin = Url::parse("http://localhost:8080").expect("Invalid RP origin URL");

    WebauthnBuilder::new(rp_id, &rp_origin)
        .expect("Failed to initialize WebAuthn")
        .build()
        .expect("Failed to build WebAuthn instance")
});

// Store sécurisé pour les passkeys
pub static CREDENTIAL_STORE: Lazy<RwLock<HashMap<String, Passkey>>> = Lazy::new(Default::default);

// Structure pour stocker l'état d'enregistrement
pub(crate) struct StoredRegistrationState {
    pub registration_state: PasskeyRegistration,
    pub challenge: String,
}

/// Démarrer l'enregistrement WebAuthn
pub async fn begin_registration(
    user_email: &str,
    user_display_name: &str,
) -> Result<(serde_json::Value, PasskeyRegistration)> {
    let user_id = Uuid::new_v4();

    let (ccr, pr) = WEBAUTHN.start_passkey_registration(
        user_id,
        user_email,
        user_display_name,
        None
    ).expect("Failed to start registration");

    Ok((
        serde_json::json!({
            "rp": ccr.public_key.rp,
            "user": {
                "id": ccr.public_key.user.id,
                "name": ccr.public_key.user.name,
                "displayName": ccr.public_key.user.display_name,
            },
            "challenge": ccr.public_key.challenge,
            "pubKeyCredParams": ccr.public_key.pub_key_cred_params,
            "timeout": ccr.public_key.timeout,
            "authenticatorSelection": ccr.public_key.authenticator_selection,
            "attestation": ccr.public_key.attestation,
        }),
        pr,
    ))
}

/// Compléter l'enregistrement WebAuthn
pub async fn complete_registration(
    user_email: &str,
    response: &RegisterPublicKeyCredential,
    stored_state: &PasskeyRegistration,
) -> Result<()> {
    // TODO
    let passkey = WEBAUTHN.finish_passkey_registration(
        response,
        &stored_state,
    ).expect("Failed to finish registration");

    set_passkey(user_email, passkey)
}

/// Démarrer l'authentification WebAuthn
pub async fn begin_authentication(user_email: &str) -> Result<(serde_json::Value, PasskeyAuthentication)> {

    let passkey = get_passkey(&user_email)?.expect("Failed to get passkey");

    // TODO
    let (rcr, pa) = WEBAUTHN.start_passkey_authentication(
        &[passkey]
    ).expect("Failed to start authentication");

    Ok((
        serde_json::json!({
            "challenge": rcr.public_key.challenge,
            "timeout": rcr.public_key.timeout,
            "rpId": rcr.public_key.rp_id,
            "allowCredentials": rcr.public_key.allow_credentials,
         }),
        pa,
    ))
}

/// Compléter l'authentification WebAuthn
pub async fn complete_authentication(
    response: &PublicKeyCredential,
    state: &PasskeyAuthentication,
    _server_challenge: &str,
) -> Result<()> {
    WEBAUTHN.finish_passkey_authentication(
        response,
        state
    ).expect("Failed to finish authentication");

    Ok(())
}
