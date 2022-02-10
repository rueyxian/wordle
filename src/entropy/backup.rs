


// ========================
// ===============================================
// ===============================================================================================

use std::collections::HashMap;

// ===============================================

fn new(words: Vec<String>) -> (WordEntropy, Occurances) {
    let mut word_entropy = HashMap::<String, Option<f64>>::with_capacity(words.len());
    let mut occurs = HashMap::<u8, HashMap<char, u64>>::new();

    words.iter().for_each(|word| {
        if word_entropy.insert(word.to_owned(), None).is_none() {
            word.chars().enumerate().for_each(|(i, c)| {
                //
                let map = occurs.entry(i as u8).or_insert(HashMap::<char, u64>::new());
                *map.entry(c).or_insert(0) += 1;
            })
        }
    });

    (WordEntropy::new(word_entropy), Occurances::new(occurs))
}

// ===============================================

#[derive(Debug)]
struct WordEntropy {
    // <k: word, v: entropy>
    map: HashMap<String, Option<f64>>,
}

impl WordEntropy {
    fn new(map: HashMap<String, Option<f64>>) -> Self {
        Self { map }
    }
}

// ===============================================

#[derive(Debug)]
struct Occurances {
    // <k: position, v: <k: letter, v: occurance>>
    map: HashMap<u8, HashMap<char, u64>>,
}

impl Occurances {
    fn new(map: HashMap<u8, HashMap<char, u64>>) -> Self {
        Self { map }
    }

    fn sorted(&self) -> Vec<(u8, Vec<(char, u64)>)> {
        todo!()
    }

    fn print(&self) {
        for (spot, occurances) in self.map.iter() {
            println!("spot: {}", spot);
            for (&letter, &occur) in occurances.iter() {
                println!("{} {}", letter, occur);
            }
            println!("");
        }
    }
}

// ===============================================

#[cfg(test)]
mod tests {

    use super::*;
    use crate::vault::WordVault;

    use crate::search::Search;

    #[test]
    fn entropy() {
        let wv = WordVault::new();
        let set = wv.read_vault().unwrap();

        // let search = Search::parse("soa| # # # ?r ?e |i").unwrap();
        let search = Search::parse("soamcyfib| # # # e r |i").unwrap();

        let (set, _) = search.search(set);

        let (words, occurances) = new(set);

        println!("words: {:?}", words);
        occurances.print();
        // println!("occurances: {:?}", occurances);
    }
}


