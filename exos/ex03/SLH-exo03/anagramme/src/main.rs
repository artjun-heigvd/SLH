use std::{error::Error, 
          io::{BufRead, BufReader},
          fs::File};
use clap::Parser;

mod utils;
use utils::{Counter, Args};


/// soit deux mots A et B;
/// normalize(A) == normalize(B) si et seulement si A et B sont des anagrammes
/// choisissez un type de retour approprié
fn normalize(s: &str) -> () {
    todo!()
}

fn main() -> Result<(), Box<dyn Error>>{

    // Pour les arguments en ligne de commande
    let args = Args::parse();

    let file = BufReader::new(File::open(args.file)?);

    for line in file.lines() {

        // Retour immédiat en cas d'erreur de lecture
        let word = line?;

        // Implémenter ça
        todo!()

    }

    // Equivalent du return 0 pour signaler à l'OS une terminaison normale
    Ok(())

}
