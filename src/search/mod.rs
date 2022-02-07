// ======================== ===============================================
// ===============================================================================================

use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

// ===============================================

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    AddSpotClashSpot(u8),
    AddSpotClashLetter(char),
    NoSpotToCompare,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO I'm lazy to do this shit
        write!(f, "TODO")
    }
}

impl std::error::Error for Error {}

// ===============================================

#[derive(Debug, Hash, Eq, Clone, Copy)]
enum Spot {
    Correct(char, u8),
    Wrong(char, u8),
    NotAny(char),
}

impl Spot {
    fn spot(&self) -> Option<u8> {
        match self {
            Spot::Correct(_, n) | Spot::Wrong(_, n) => Some(*n),
            Spot::NotAny(_) => None,
        }
    }

    fn letter(&self) -> char {
        match self {
            Spot::Correct(c, _) | Spot::Wrong(c, _) => *c,
            Spot::NotAny(c) => *c,
        }
    }

    fn eq_spot(&self, other: &Self) -> Result<bool, Error> {
        match (self.spot(), other.spot()) {
            (None, None) | (None, Some(_)) | (Some(_), None) => Err(Error::NoSpotToCompare),
            (Some(a), Some(b)) => Ok(a == b),
        }
    }

    fn eq_letter(&self, other: &Self) -> bool {
        self.letter() == other.letter()
    }
}

impl PartialEq for Spot {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Correct(_, n1), Self::Correct(_, n2)) => n1 == n2,
            (Self::Wrong(_, n1), Self::Wrong(_, n2)) => n1 == n2,
            (Self::NotAny(c1), Self::NotAny(c2)) => c1 == c2,
            _ => false,
        }
    }
}

// ===============================================

#[derive(Default, Debug)]
pub struct Search {
    spots: HashSet<Spot>,
    isogram: bool,
}

impl Search {
    pub fn parse(s: &str) -> Result<Self, Error> {
        let mut spots = Vec::<Spot>::new();
        let s = s.to_lowercase();

        let regex = Regex::new(r"^([a-z]*)\|((?: (?:#|[a-z]|\?[a-z])){5}) \|(i?)$").unwrap();

        let caps = regex
            .captures(s.as_str())
            .ok_or(Error::ParseError(s.to_owned()))?;
        let no_spot_letters = caps.get(1).map_or("", |m| m.as_str());
        let yes_spot_letters = caps.get(2).map_or("", |m| m.as_str()).trim_start();
        let extras = caps.get(3).map_or("", |m| m.as_str());

        for letter in no_spot_letters.chars() {
            spots.push(Spot::NotAny(letter));
        }

        for (i, letters) in yes_spot_letters.split_whitespace().enumerate() {
            let mut iter = letters.chars();
            match iter.next() {
                Some(letter @ 'a'..='z') => spots.push(Spot::Correct(letter, i as u8)),
                Some('?') => spots.push(Spot::Wrong(iter.next().unwrap(), i as u8)),
                Some('#') => continue,
                _ => unreachable!(),
            }
        }

        let mut search = Search::new(extras == "i");
        search.add_spots(spots)?;

        Ok(search)
    }

    pub fn new(isogram: bool) -> Self {
        Self {
            isogram,
            ..Default::default()
        }
    }

    fn add_spots(&mut self, spots: Vec<Spot>) -> Result<(), Error> {
        for spot in spots {
            if let Some(n) = spot.spot() {
                self.spots
                    .iter()
                    .filter(|candidate| candidate.spot().is_some())
                    .all(|candidate| {
                        if let Ok(is_eq) = spot.eq_spot(candidate) {
                            !is_eq
                        } else {
                            unreachable!()
                        }
                    })
                    .then(|| ())
                    .ok_or(Error::AddSpotClashSpot(n))?;
            } else {
                self.spots
                    .iter()
                    .filter(|candidate| candidate.spot().is_none())
                    .all(|candidate| !spot.eq_letter(candidate))
                    .then(|| ())
                    .ok_or(Error::AddSpotClashLetter(spot.letter()))?;
            }
            self.spots.insert(spot);
        }
        Ok(())
    }

    fn get_correct_spots(&self) -> HashMap<char, u8> {
        self.spots
            .iter()
            .filter(|spot| match spot {
                Spot::Correct(_, _) => true,
                _ => false,
            })
            .map(|spot| match spot {
                Spot::Correct(c, n) | Spot::Wrong(c, n) => (*c, *n),
                Spot::NotAny(_) => unreachable!(),
            })
            .collect::<HashMap<char, u8>>()
    }

    fn get_wrong_spots(&self) -> HashMap<char, u8> {
        self.spots
            .iter()
            .filter(|spot| match spot {
                Spot::Wrong(_, _) => true,
                _ => false,
            })
            .map(|spot| match spot {
                Spot::Correct(c, n) | Spot::Wrong(c, n) => (*c, *n),
                Spot::NotAny(_) => unreachable!(),
            })
            .collect::<HashMap<char, u8>>()
    }

    fn get_not_any_spots(&self) -> HashSet<char> {
        self.spots
            .iter()
            .filter(|spot| match spot {
                Spot::NotAny(_) => true,
                _ => false,
            })
            .map(|spot| spot.letter())
            .collect::<HashSet<char>>()
    }

    fn is_isogram(s: &str) -> bool {
        let mut set = HashSet::with_capacity(s.len());
        s.chars().all(|c| set.insert(c))
    }

    pub fn search(&self, set: HashSet<String>) -> (HashSet<String>, bool) {
        let correct_spot = self.get_correct_spots();
        let wrong_spot = self.get_wrong_spots();
        let not_any_spot = self.get_not_any_spots();

        let set = if self.isogram {
            set.into_iter()
                .filter(|word| Search::is_isogram(word))
                .collect::<HashSet<String>>()
        } else {
            set
        };

        let result = set
            .into_iter()
            .filter(|word| {
                let word_bytes = word.as_bytes();

                if not_any_spot
                    .iter()
                    .any(|&c| word_bytes.contains(&(c as u8)))
                {
                    return false;
                }

                // TODO: handle `array out of bound` properly
                if correct_spot
                    .iter()
                    .any(|(&c, &n)| word_bytes[n as usize] as char != c)
                {
                    return false;
                }

                if wrong_spot.iter().any(|(&c, &n)| {
                    (word_bytes[n as usize] as char == c)
                        || word
                            .chars()
                            .enumerate()
                            .filter(|(i, _)| *i != n as usize)
                            .all(|(_, c0)| c0 != c)
                }) {
                    return false;
                }

                return true;
            })
            .collect::<HashSet<String>>();

        (result, self.isogram)
    }
}

// ===============================================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_empty() {
        let search = Search::parse("| # # # # # |").unwrap();
        assert_eq!(search.spots.len(), 0);
        assert_eq!(search.isogram, false);
    }

    #[test]
    fn parse() {
        let search = Search::parse("abcd| # ?e f # ?g |i").unwrap();
        assert_eq!(search.spots.len(), 7);
        assert_eq!(search.isogram, true);
    }

    #[test]
    fn parse_error() {
        assert!(Search::parse(" # # # # #").is_err());
        assert!(Search::parse("abcde| #?e f # |i").is_err());
        assert!(Search::parse("| # ?e f # ?g ?h|lulz").is_err());
        assert!(Search::parse("abcde| #?e f # ?g ?h").is_err());
    }
}
