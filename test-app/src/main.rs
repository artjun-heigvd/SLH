#[macro_use] use rocket::*;

use biscuit_auth::{Biscuit, KeyPair};
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::{sync::LazyLock, time::{Duration, SystemTime}};

// Structure pour l'envoi/réception du biscuit en JSON
#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

lazy_static::lazy_static! {
    static ref KEYPAIR: KeyPair = KeyPair::new();
}

// 🔑 Génère un biscuit pour un utilisateur donné
#[post("/generate/<username>")]
fn generate_biscuit(username: String) -> Json<TokenResponse> {
    
    // Création d'un biscuit avec un fait utilisateur
    let mut builder = Biscuit::builder();
    builder.add_fact(format!("user(\"{}\")", username).as_str())
        .expect("Erreur lors de la création du biscuit");
    let biscuit = builder.build(&KEYPAIR)
        .expect("Erreur de signature");

    let token = biscuit.to_base64().expect("Erreur d'encodage");
    Json(TokenResponse { token })
}

// 🔍 Vérifie si le biscuit donné permet une opération
#[post("/verify", format = "json", data = "<token_json>")]
fn verify_biscuit(token_json: Json<TokenResponse>) -> &'static str {

    // Log the received token
    println!("Received token: {}", token_json.token);
    
    // Décoder le biscuit reçu
    let biscuit = match Biscuit::from_base64(&token_json.token, &KEYPAIR.public()) {
        Ok(b) => b,
        Err(e) => {
            println!("Error decoding biscuit: {:?}", e);
            return "Biscuit invalide";
        },
    };

    // Définir un authorizer avec un contexte (ex: action demandée)
    let mut authorizer = biscuit.authorizer().expect("Erreur de création de l'authorizer");
    authorizer.add_fact("operation(\"read\")").expect("Erreur d'ajout de fact");

    // Ajouter une règle qui autorise l'opération si l'utilisateur est défini
    //authorizer.add_rule("ok() <- user($u), operation(\"read\");")
    //    .expect("Erreur d'ajout de règle");

    // Vérifier l'autorisation
    match authorizer.authorize() {
        Ok(_) => "Autorisation accordée",
        Err(_) => "Accès refusé",
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![generate_biscuit, verify_biscuit])
}
