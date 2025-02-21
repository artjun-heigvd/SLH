use std::{collections::BTreeMap, env, fs::File, sync::LazyLock, time::Instant};

use anyhow::{anyhow, Result};
use regex::Regex;
use serde::Deserialize;
use smallvec::SmallVec;

/// MovieRecord représente une ligne de CSV en mémoire.
///
/// Nous prenons la propriété de ces strings parce que les
/// éventuels escapes du format CSV nous forcent à faire
/// une copie quoi qu'il arrive.
#[derive(Debug, Deserialize)]
struct MovieRecord {
    #[serde(rename = "movieId")]
    _movie_id: u32,
    title: String,
    genres: String,
}

/// Movie représente les informations extraites d'un MovieRecord.
///
/// Cette fois, il n'est pas nécessaire de copier quoi que ce soit,
/// nous allons donc référencer les données du MovieRecord original.
///
/// Il faut donc un lifetime qui indique pendant combien de temps
/// le MovieRecord original a été emprunté.
#[allow(dead_code)]
#[derive(Debug)]
struct Movie<'r> {
    title: &'r str,
    year: Option<Year>,
    genres: SmallVec<[&'r str; 8]>,
}

// LazyLock permet d'initialiser un global statique à sa première utilisation
static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Unwrap est OK ici parce qu'une erreur de compilation de la regex
    // indique à 100% un problème de notre code
    Regex::new(r"\(([0-9]{4})\)").expect("regex compilation failed")
});

fn extract_year(title: &str) -> Option<Year> {

    let re = Regex::new("^.*\d\d\d\d.*$").unwrap();

    let res = re.captures(title);

    Option::from(res)
}

impl MovieRecord {
    fn extract(&self) -> Movie {
        // Hint: .collect() peut construire un SmallVec
        todo!("découper les genres en une liste")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Year(u16);

#[derive(Debug, Default)]
struct Stats {
    total: u32,
    total_by_year: BTreeMap<Year, u32>,
}

impl Stats {
    fn update(&mut self, movie: &Movie) {
        todo!("ajouter le film aux compteurs")
    }
}

fn run() -> Result<()> {
    let genre = env::args()
        .nth(1)
        .ok_or(anyhow!("expected 1 argument, but got none"))?;

    let file: File = File::open("movies.csv")?;
    let mut csv_reader = csv::Reader::from_reader(file);

    let mut stat: Stats = todo!("");

    for maybe_record in csv_reader.deserialize::<MovieRecord>() {
   
        todo!("extraire un Movie et mettre a jour les stats avec");

    }

    println!("{:?}", stat);

    Ok(())
}

fn main() -> Result<()> {
    let now = Instant::now();

    run()?;

    println!("Completed successfully in {} ms", now.elapsed().as_millis());

    Ok(())
}
