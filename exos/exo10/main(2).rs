use dryoc::generichash::{GenericHash, Hash, Key};
use dryoc::sign::*;
use rand::{thread_rng, Rng};
use read_input::prelude::*;

//We will draw a value between 1 and MAX
const MAX: u8 = 5;

/// Ask client to draw a value between 1 and MAX
/// Commit to this value
/// Return the secret draw, the hash digest, and the randomness used during commit)
fn client_secret() -> (u8, Hash, [u8; 16]) {
    let secret: u8 = input()
        .inside(1..=MAX)
        .msg(format!(
            "Enter your secret number between 1 and {} (included): ",
            MAX
        ))
        .get();
    let mut randomness = [0u8; 16];
    thread_rng().fill(&mut randomness[..]);
    let mut hash_input: Vec<u8> = randomness.to_vec();
    hash_input.push(secret);
    let digest: Hash =
        GenericHash::hash_with_defaults::<_, Key, _>(&hash_input, None).expect("hash failed");
    (secret, digest, randomness)
}

///Draws uniformly a number between 1 and MAX and sign this value
///Return the signed message and the value drawn
fn lottery_draw(
    keypair: &SigningKeyPair<PublicKey, SecretKey>,
) -> (SignedMessage<Signature, Message>, u8) {
    let mut rng = thread_rng();
    let draw: u8 = rng.gen_range(1..=MAX);
    //We return the signed message and the draw
    (
        keypair.sign_with_defaults([draw]).expect("signing failed"),
        draw,
    )
}

///Verify that the user really won.
///Recompute the hash and confirm win if they are equal
fn proof(randomness: &[u8; 16], draw: u8, hash: &Hash) {
    let mut hash_input: Vec<u8> = randomness.to_vec();
    hash_input.push(draw);
    let digest: Hash =
        GenericHash::hash_with_defaults::<_, Key, _>(&hash_input, None).expect("hash failed");
    if digest == *hash {
        println!("Digests are the same. You really won!");
    } else {
        println!("Cheater!");
    }
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
    if secret == draw && signed_message.verify(&keypair.public_key).is_ok() {
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
