//! Gestion des routes accessibles sans authentification.
//! Contient les handlers pour les pages publiques, l'inscription, la connexion,
//! la récupération de compte et la validation d'utilisateur.

use axum::extract::{Path, Json, Query};
use axum::response::{Redirect, IntoResponse, Html};
use axum::http::StatusCode;
use once_cell::sync::Lazy;
use serde_json::json;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tower::ServiceExt;
use uuid::Uuid;
use webauthn_rs::prelude::{PasskeyAuthentication, PasskeyRegistration, PublicKeyCredential, RegisterPublicKeyCredential};
use crate::HBS;
use crate::database::{user, token};
use crate::email::send_mail;
use crate::utils::webauthn::{begin_registration, complete_registration, begin_authentication, complete_authentication, StoredRegistrationState, CREDENTIAL_STORE};
use crate::utils::input::{UserEmail, Username};

/// Structure pour gérer un état temporaire avec un challenge
struct TimedStoredState<T> {
    state: T,
    server_challenge: String,
}

/// Stockage des états d'enregistrement et d'authentification
pub(crate) static REGISTRATION_STATES: Lazy<RwLock<HashMap<String, PasskeyRegistration>>> =
    Lazy::new(Default::default);
static AUTHENTICATION_STATES: Lazy<RwLock<HashMap<String, TimedStoredState<PasskeyAuthentication>>>> = Lazy::new(Default::default);

/// Début du processus d'enregistrement WebAuthn
pub async fn register_begin(Json(payload): Json<serde_json::Value>) -> axum::response::Result<Json<serde_json::Value>> {
    let email_raw = payload
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::BAD_REQUEST, "Email is required"))?;

    // Validate email
    let email = UserEmail::try_new(email_raw)
        .ok_or((StatusCode::BAD_REQUEST, "Invalid email address"))?;
    let email_str = email.as_ref();

    let reset_mode = payload.get("reset_mode").and_then(|v| v.as_bool()).unwrap_or(false);

    // TODO: Perform any necessary pre-registration steps here

    let (pk, pr) = begin_registration(email_str, email_str).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start registration"))?;

    let state_id = Uuid::new_v4().to_string();
    REGISTRATION_STATES.write().await.insert(state_id.clone(), pr);

    Ok(Json(json!({
        "publicKey": pk,
        "state_id": state_id,
    })))
}

/// Fin du processus d'enregistrement WebAuthn
pub async fn register_complete(Json(payload): Json<serde_json::Value>) -> axum::response::Result<StatusCode> {
    let email_raw = payload
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::BAD_REQUEST, "Email is required"))?;

    // Validate email
    let email = UserEmail::try_new(email_raw)
        .ok_or((StatusCode::BAD_REQUEST, "Invalid email address"))?;
    let email_str = email.as_ref();

    let state_id = payload
        .get("state_id")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::BAD_REQUEST, "State ID is required"))?;

    let reset_mode = payload.get("reset_mode").and_then(|v| v.as_bool()).unwrap_or(false);

    let stored_state = REGISTRATION_STATES.write().await.remove(state_id)
        .ok_or((StatusCode::BAD_REQUEST, "Invalid state ID"))?;

    let cred = payload
        .get("response")
        .and_then(|v| serde_json::from_value::<RegisterPublicKeyCredential>(v.clone()).ok())
        .ok_or((StatusCode::BAD_REQUEST, "Invalid response"))?;

    complete_registration(email_str, &cred, &stored_state).await
        .map_err(|_| (StatusCode::FORBIDDEN, "Failed to complete registration"))?;

    let passkey = CREDENTIAL_STORE.read().await.get(email_str).unwrap().clone();

    // TODO: Perform any necessary post-registration steps here

    user::set_passkey(email_str, passkey)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to complete registration"))?;

    Ok(StatusCode::OK)
}

/// Début du processus d'authentification WebAuthn
pub async fn login_begin(Json(payload): Json<serde_json::Value>) -> axum::response::Result<Json<serde_json::Value>> {
    let email_raw = payload
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::BAD_REQUEST, "Email is required"))?;

    // Validate email
    let email = UserEmail::try_new(email_raw)
        .ok_or((StatusCode::BAD_REQUEST, "Invalid email address"))?;
    let email_str = email.as_ref();

    let (public_key, auth_state) = begin_authentication(email_str).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start authentication"))?;

    let state_id = Uuid::new_v4().to_string();
    AUTHENTICATION_STATES.write().await.insert(state_id.clone(), TimedStoredState {
        state: auth_state,
        server_challenge: public_key["challenge"].as_str().unwrap().to_string(),
    });

    Ok(Json(json!({
        "publicKey": public_key,
        "state_id": state_id,
    })))
}

/// Fin du processus d'authentification WebAuthn
pub async fn login_complete(Json(payload): Json<serde_json::Value>) -> axum::response::Result<Redirect> {
    let state_id = payload
        .get("state_id")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::BAD_REQUEST, "State ID is required"))?;

    let response: PublicKeyCredential = serde_json::from_value(payload.get("response").cloned().unwrap())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid response format"))?;

    let stored_state = AUTHENTICATION_STATES.write().await.remove(state_id)
        .ok_or((StatusCode::BAD_REQUEST, "Invalid state ID"))?;

    complete_authentication(&response, &stored_state.state, &stored_state.server_challenge).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to complete authentication"))?;

    Ok(Redirect::to("/home"))
}

/// Gère la déconnexion de l'utilisateur
pub async fn logout() -> impl IntoResponse {
    Redirect::to("/")
}

/// Valide un compte utilisateur via un token
pub async fn validate_account(Path(token): Path<String>) -> impl IntoResponse {
    match token::consume(&token) {
        Ok(email) => match user::verify(&email) {
            Ok(_) => Redirect::to("/login?validated=true"),
            Err(_) => Redirect::to("/register?error=validation_failed"),
        },
        Err(_) => Redirect::to("/register?error=invalid_token"),
    }
}

/// Envoie un email de récupération de compte à l'utilisateur
pub async fn recover_account(Json(payload): Json<serde_json::Value>) -> axum::response::Result<Html<String>> {
    let email_raw = payload.get("email").and_then(|v| v.as_str()).ok_or((StatusCode::BAD_REQUEST, "Email is required"))?;

    // Validate email
    let email = UserEmail::try_new(email_raw)
        .ok_or((StatusCode::BAD_REQUEST, "Invalid email address"))?;
    let email_str = email.as_ref();

    let token = token::generate(email_str).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate token"))?;

    // Use the send_mail function to send the recovery email
    let subject = "Account Recovery";
    let body = format!("Please use the following link to recover your account: http://localhost:8080/reset_account/{}", token);
    send_mail(email_str, &subject, &body).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send recovery email"))?;

    HBS.render("recover", &json!({"success": true}))
        .map(Html)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.").into())
}

/// Gère la réinitialisation du compte utilisateur via un token de récupération
pub async fn reset_account(Path(token): Path<String>) -> Html<String> {
    match token::consume(&token) {
        Ok(email) => {
            let redirect_url = format!("/register?reset_mode=true&email={}&success=true", email);
            Html(format!("<meta http-equiv='refresh' content='0;url={}'/>", redirect_url))
        }
        Err(_) => {
            let redirect_url = "/register?error=recovery_failed";
            Html(format!("<meta http-equiv='refresh' content='0;url={}'/>", redirect_url))
        }
    }
}

/// --- Affichage des pages ---
///
/// Affiche la page d'accueil
pub async fn index(session: tower_sessions::Session) -> impl IntoResponse {
    let is_logged_in = session.get::<String>("email").is_ok();
    let mut data = HashMap::new();
    data.insert("logged_in", is_logged_in);

    HBS.render("index", &data)
        .map(Html)
        .unwrap_or_else(|_| Html("Internal Server Error".to_string()))
}

/// Affiche la page de connexion
pub async fn login_page() -> impl IntoResponse {
    Html(include_str!("../../templates/login.hbs"))
}

/// Affiche la page d'inscription avec des messages contextuels si présents
pub async fn register_page(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let mut context = HashMap::new();
    if let Some(success) = params.get("success") {
        if success == "true" {
            context.insert("success_message", "Account recovery successful. Please reset your passkey.");
        }
    }
    if let Some(error) = params.get("error") {
        if error == "recovery_failed" {
            context.insert("error_message", "Invalid or expired recovery link. Please try again.");
        }
    }

    HBS.render("register", &context)
        .map(Html)
        .unwrap_or_else(|_| Html("<h1>Internal Server Error</h1>".to_string()))
}

/// Affiche la page de récupération de compte
pub async fn recover_page() -> impl IntoResponse {
    Html(include_str!("../../templates/recover.hbs"))
}
