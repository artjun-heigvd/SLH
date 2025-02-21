use dryoc::generichash::{GenericHash, Hash, Key};
use dryoc::sign::*;
use rand::{rngs::OsRng, Rng};
use read_input::prelude::*;

//We will draw a value between 1 and MAX
const MAX: u8 = 5;

/// Ask client to draw a value between 1 and MAX
/// Commit to this value
/// Return the secret draw, the hash digest, and the randomness used during commit)
fn client_secret() -> (u8, Hash, [u8; 16]) {
    todo!()
}

///Draws uniformly a number between 1 and MAX and sign this value
///Return the signed message and the value drawn
fn lottery_draw(
    keypair: &SigningKeyPair<PublicKey, SecretKey>,
) -> (SignedMessage<Signature, Message>, u8) {
    todo!()
}

///Verify that the user really won.
///Recompute the hash and confirm win if they are equal
fn proof(randomness: &[u8; 16], draw: u8, hash: &Hash) {
    todo!()
}

fn main() {
    //key pair to sign and verify lottery draws
    let keypair = SigningKeyPair::gen_with_defaults();
    //The client commits to its secret. Only the hash is broadcast
    let (secret, hash, randomness) = client_secret();
    //print the hash so that we cannot change it
    println!("Here is my hash. I cannot cheat: {:?}", hex::encode(&hash));
    let (signed_message, draw) = lottery_draw(&keypair);
    println!("The winning number is {}", draw);
    //verify signature and check if you won
    if todo!() { //TODO
        println!("You won!");
        println!(
            "Here is my randomness. I didn't cheat: {:?}",
            hex::encode(&randomness)
        );
        proof(&randomness, draw, &hash);
    } else {
        println!("You lost!");
    }
}
