// Faster than std String: strings < 23 bytes are stored inline
// improves cache locality and memory allocator pressure
use smartstring::alias::String;

use std::cell::{RefCell, RefMut};

// For reading data
use std::{fs::File, io::{BufRead, BufReader}};

// For static initialization of regexes
use std::sync::LazyLock;

// For the contact-tracing algorithm
use std::collections::{BTreeMap, BTreeSet};

use regex::{self, Regex};

// Better performance on tiny vectors
use smallvec::SmallVec;

use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Person(String);

#[derive(Debug)]
enum Event {
    Positive { who: Person },
    Negative { who: Person },
    Contact { from: Person, to: Person, },
    Bidirectional { one: Person, two: Person, }
}


fn persons(s: &str) -> SmallVec<[Person; 2]> {
    static PERSON_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b[A-Z][a-z]+ [A-Z]+(?: [A-Z]+)?\b").unwrap()
    });

    PERSON_REGEX
        .find_iter(s)
        .map(|m| Person(m.as_str().to_string().parse().unwrap()))
        .collect()
}

fn parse_event(evt: &str) -> Option<Event> {
    if evt.starts_with("---") {
        if let Some(person) = persons(evt).get(0) {
            return Some(Event::Negative {
                who: person.clone(),
            });
        }
    } else if evt.starts_with("+++") {
        if let Some(person) = persons(evt).get(0) {
            return Some(Event::Positive {
                who: person.clone(),
            });
        }
    } else if evt.starts_with("-->") {
        if let Some(persons) = persons(evt).get(0..2) {
            return Some(Event::Contact {
                from: persons[0].clone(),
                to: persons[1].clone(),
            });
        }
    } else if evt.starts_with("<->") {
        if let Some(persons) = persons(evt).get(0..2) {
            return Some(Event::Bidirectional {
                one: persons[0].clone(),
                two: persons[1].clone(),
            });
        }
    }

    None
}


// Parsing à la volée, pour ne pas avoir besoin de charger
// tous les events en mémoire d'un coup.
fn load_events() -> Result<impl Iterator<Item=Event>> {
    let file = File::open("events.txt")?;
    let buf = BufReader::new(file);
    Ok(buf.lines()
        .filter_map(|l| l.ok().and_then(|s| parse_event(&s))))
}

fn main() -> Result<()> {
    let events = load_events()?;

    for evt in events {
        println!("{:?}", evt);
    }

    Ok(())
}

