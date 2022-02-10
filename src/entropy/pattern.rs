// ========================
// ===============================================
// ===============================================================================================

use regex::Regex;

// ===============================================

#[derive(Debug)]
pub enum Error {
    TryFromStr(String),
    TryFromStrStr(String, String),
    AddPattern,
}

// ===============================================

#[derive(Debug, Clone)]
pub enum Unit {
    Correct(char),
    Wrong(char),
    NotAny(char),
}

// ===============================================

#[derive(Debug, Clone)]
pub struct Pattern {
    pub units: Vec<Unit>,
}

impl Pattern {
    // pub fn units_len(&self) -> usize {
    //     self.units.len()
    // }
}

impl TryFrom<&str> for Pattern {
    type Error = crate::entropy::pattern::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s = value.to_lowercase();
        let regex =
            Regex::new(r"^(?P<units>(?:#[a-z]|\?[a-z]|![a-z])(?:\s(?:#[a-z]|\?[a-z]|![a-z]))*)$")
                .unwrap();
        let caps = regex
            .captures(s.as_str())
            .ok_or(Error::TryFromStr(value.to_owned()))?;
        let cap = caps.name("units").map_or("", |m| m.as_str()).trim_start();
        let mut units = Vec::<Unit>::with_capacity(cap.len());
        for letters in cap.split_whitespace() {
            let mut letters_iter = letters.chars();
            match letters_iter.next() {
                // Some(letter @ 'a'..='z') => units.push(Unit::Correct(letter)),
                Some('!') => units.push(Unit::Correct(letters_iter.next().unwrap())),
                Some('?') => units.push(Unit::Wrong(letters_iter.next().unwrap())),
                Some('#') => units.push(Unit::NotAny(letters_iter.next().unwrap())),
                _ => unreachable!(),
            };
        }
        Ok(Pattern { units })
    }
}

impl TryFrom<(&str, &str)> for Pattern {
    type Error = crate::entropy::pattern::Error;
    fn try_from((word, pattern): (&str, &str)) -> Result<Self, Self::Error> {
        if word.len() != pattern.len() {
            return Err(Error::TryFromStrStr(word.to_owned(), pattern.to_owned()));
        }
        let mut units = Vec::<Unit>::with_capacity(word.len());
        for (c, u) in word.to_lowercase().chars().zip(pattern.chars()) {
            units.push(match u {
                '!' => Unit::Correct(c),
                '?' => Unit::Wrong(c),
                '#' => Unit::NotAny(c),
                _ => return Err(Error::TryFromStrStr(word.to_owned(), pattern.to_owned())),
            });
        }
        Ok(Pattern { units })
    }
}

// ===============================================

#[derive(Debug, Default, Clone)]
pub struct PatternStack {
    unit_count: usize,
    stack: Vec<Pattern>,
}

impl PatternStack {
    pub fn new(unit_count: usize) -> Self {
        Self {
            unit_count,
            ..Default::default()
        }
    }

    pub fn add_pattern(&mut self, pattern: Pattern) -> Result<(), Error> {
        (pattern.units.len() == self.unit_count.into())
            .then(|| ())
            .ok_or(Error::AddPattern)?;
        self.stack.push(pattern);
        Ok(())
    }


    fn is_possible_word(&self, word: &str) -> bool {
        for pattern in self.stack.iter() {
            for (i, unit) in pattern.units.iter().enumerate() {
                if match unit {
                    Unit::Correct(c) => word.chars().nth(i).unwrap() != *c,
                    Unit::Wrong(c) => {
                        word.chars().nth(i).unwrap() == *c
                            || word
                                .chars()
                                .enumerate()
                                .filter(|(i0, _)| i != *i0)
                                .all(|(_, c0)| c0 != *c)
                    }
                    Unit::NotAny(c) => word.chars().any(|c0| *c == c0),
                } {
                    return false;
                }
            }
        }
        true
    }

    pub fn possible_words(&self, words: &Vec<String>) -> Vec<String> {
        words
            .into_iter()
            .filter(|word| self.is_possible_word(word))
            .cloned()
            .collect::<Vec<String>>()
    }

    pub fn possible_word_count(&self, words: &Vec<String>) -> u64 {
        words
            .iter()
            .filter(|word| self.is_possible_word(word))
            .count() as u64
    }

    // pub fn calculate_info_qty(&self, words: Vec<String>) -> f64 {
    //     let possible_words = self.possible_words(&words);
    //     let probability = possible_words.len() as f64 / words.len() as f64;
    //     let info_qty = -probability.log2();
    //     info_qty
    // }

    // pub fn information(&self, words: Vec<String>) -> (Vec<String>, f64) {
    //     let possible_words = self.possible_words(&words);
    //     let probability = possible_words.len() as f64 / words.len() as f64;
    //     let information = -probability.log2();
    //     (possible_words, information)
    // }

    // fn possible_next_patterns(current: PatternVec, word: &str) -> HashSet<PatternVec> {
    //     todo!()
    // }
}

// ===============================================================================================

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn try_from_str() {
    //     let pattern = Pattern::try_from("#a #b #c #d #e").unwrap();
    //     println!("{:?}", pattern);
    //
    //     let pattern = Pattern::try_from("#a ?b f #d ?e").unwrap();
    //     println!("{:?}", pattern);
    // }

    // #[test]
    // fn try_from_str_str() {
    //     let pattern = Pattern::try_from(("corgi", "?#!?#")).unwrap();
    //     println!("{:?}", pattern);
    // }

    // ===============================================
}
