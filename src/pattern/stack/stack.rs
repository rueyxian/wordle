// ===============================================

use super::line;
use super::unit;

// ===============================================

#[derive(Debug)]
pub enum Error {
    AddPatternLine,
}

// ===============================================

#[derive(Debug, Default, Clone)]
pub struct PatternStack {
    unit_count: usize,
    stack: Vec<line::PatternLine>,
}

impl PatternStack {
    pub fn new(unit_count: usize) -> Self {
        Self {
            unit_count,
            // ..Default::default()
            //FIXME: fix this stupid declaration
            stack: Vec::with_capacity(6),
        }
    }

    pub fn progress(&mut self, line: line::PatternLine) -> Result<(), Error> {
        (line.units.len() == self.unit_count.into())
            .then(|| ())
            .ok_or(Error::AddPatternLine)?;
        self.stack.push(line);
        Ok(())
    }

    pub fn revert(&mut self) -> Option<line::PatternLine> {
        self.stack.pop()
    }

    fn is_possible_word(&self, word: &str) -> bool {
        for pattern_line in self.stack.iter() {
            for (i, unit) in pattern_line.units.iter().enumerate() {
                if match unit {
                    unit::Unit::Correct(c) => word.chars().nth(i).unwrap() != *c,
                    unit::Unit::Wrong(c) => {
                        word.chars().nth(i).unwrap() == *c
                            || word
                                .chars()
                                .enumerate()
                                .filter(|(i0, _)| i != *i0)
                                .all(|(_, c0)| c0 != *c)
                    }
                    unit::Unit::NotAny(c) => word.chars().any(|c0| {
                        // Repeated letters on the same line, one of them is correct.
                        // For example: #r ?u ?l !e !r
                        // Eventhough, the first letter `r` is marked as `not any`,
                        // we shouldn't rule out any words that contains `r` in the word set.
                        let corner_case = pattern_line.units.iter().any(|unit| match unit {
                            unit::Unit::Correct(c1) => c0 == *c1,
                            unit::Unit::Wrong(_) | unit::Unit::NotAny(_) => false,
                        });
                        c0 == *c && !corner_case
                    }),
                } {
                    return false;
                }
            }
        }
        true
    }

    pub fn possible_words(&self, words: &Vec<String>) -> Vec<String> {
        words
            .iter()
            .filter(|&word| self.is_possible_word(word))
            .cloned()
            .collect::<Vec<String>>()
    }

    // pub fn possible_words<'a>(&self, words: &Vec<&'a str>) -> Vec<&'a str> {
    //     words
    //         .iter()
    //         .filter(|word| self.is_possible_word(word))
    //         .cloned()
    //         .collect::<Vec<&str>>()
    // }

    // pub fn possible_word_count(&self, words: &Vec<&str>) -> u64 {
    //     words
    //         .iter()
    //         .filter(|word| self.is_possible_word(word))
    //         .count() as u64
    // }

    pub fn possible_word_count(&self, words: &Vec<String>) -> u64 {
        words
            .iter()
            .filter(|word| self.is_possible_word(word))
            .count() as u64
    }
}

impl std::fmt::Display for PatternStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.stack.iter() {
            write!(f, "  {}\n", line)?;
        }
        Ok(())
    }
}
