use std::{collections::HashMap, ops::AddAssign, path::PathBuf};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    /// Mot à chercher
    pub target: String,

    /// Dictionnaire
    #[arg(short='f', value_name = "FILE", default_value="./wordlist.txt")]
    pub file: PathBuf,

    ///// Pattern
    ///// "-p1 -p3 -p2" produira une valeur Some([1,3,2])
    // #[arg(short='p')]
    // pub pattern: Option<Vec<usize>>

}

/// Counts the number of occurrences of each character
/// in a string.
/// 
/// Can be compared directly with `==`, can also be added with (+)
#[derive(Clone,Default,Debug,PartialEq,Eq)]
pub struct Counter(HashMap<char,usize>);

impl Counter {

    pub fn from(s: &str) -> Self {
        Counter::default() + s
    }

}

impl std::ops::AddAssign<&str> for Counter {
    fn add_assign(&mut self, rhs: &str) {
        for c in rhs.chars() {
            *self.0.entry(c).or_default() += 1;
        }
    }
}

impl std::ops::Add<&str> for Counter {
    type Output = Self;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl std::ops::Sub<&str> for Counter {
    type Output = Option<Self>;
    
    fn sub(self, rhs: &str) -> Self::Output {
        let mut result = self.clone();

        for c in rhs.chars() {
            let current = result.0.get_mut(&c)?;
            *current = current.checked_sub(1)?;
            if *current == 0 {
                result.0.remove(&c);
            }
        }
        Some(result)
    }
    
}

#[test]
fn basic_test() {

    let a = Counter::from("abcd");
    let b = Counter::from("1234");
    let c = Counter::from("1a2b3c4d");

    assert_ne!(&a, &b);
    assert_ne!(&a, &c);
    assert_eq!(&(a+"4312"), &c);

    assert_eq!(c - "dcab", Some(b));

}