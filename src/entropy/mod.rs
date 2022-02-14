// ========================
// ===============================================
// ===============================================================================================

use super::pattern;
use crate::cli;

// ===============================================

// #[derive(Debug, Clone)]
// struct WordEntropy<'a> {
//     word: &'a str,
//     bit: f64,
// }
//
// impl<'a> WordEntropy<'a> {
//     fn new(
//         word: &'a str,
//         pattern_perm: &Vec<String>,
//         curr_pattern_stack: &mut pattern::PatternStack,
//         // curr_words: &Vec<String>,
//         curr_words: &Vec<&str>,
//     ) -> Self {
//         // use rayon::prelude::*;
//         // let curr_pattern_stack = Mutex::new(curr_pattern_stack);
//         // let sum_probability = Mutex::new(0.0_f64);
//         // let occurance = Mutex::new(0_u64);
//         //
//         // pattern_perm.par_iter().for_each(|pattern_str| {
//         //     let pattern = pattern::PatternLine::try_from((word, pattern_str.as_str())).unwrap();
//         //
//         //     (*curr_pattern_stack.lock().unwrap())
//         //         .add_pattern_line(pattern)
//         //         .unwrap();
//         //
//         //     let probability = (*curr_pattern_stack.lock().unwrap()).possible_word_count(curr_words)
//         //         as f64
//         //         / curr_words.len() as f64;
//         //
//         //     (*curr_pattern_stack.lock().unwrap()).remove_last_pattern_line();
//         //
//         //     if probability.abs() < f64::EPSILON {
//         //         *occurance.lock().unwrap() += 1;
//         //     }
//         //
//         //     *sum_probability.lock().unwrap() += probability;
//         // });
//
//         // ========================
//
//         let mut sum_probability: f64 = 0.0;
//
//         let mut occurance = 0;
//
//         for pattern_str in pattern_perm.iter() {
//             let pattern = pattern::PatternLine::try_from((word, pattern_str.as_str())).unwrap();
//
//             curr_pattern_stack.add_pattern_line(pattern).unwrap();
//
//             let probability =
//                 curr_pattern_stack.possible_word_count(curr_words) as f64 / curr_words.len() as f64;
//
//             curr_pattern_stack.remove_last_pattern_line();
//
//             if probability.abs() < f64::EPSILON {
//                 occurance += 1;
//             }
//
//             sum_probability += probability;
//         }
//
//         // ========================
//
//         let probability = sum_probability / occurance as f64;
//         // let probability = *sum_probability.lock().unwrap() / *occurance.lock().unwrap() as f64;
//
//         let bit = -(probability.log2());
//
//         Self { word, bit }
//     }
// }

// ===============================================

#[derive(Debug, Clone)]
pub struct EntropyUnit {
    word: String,
    entropy: f64,
}

impl EntropyUnit {
    fn new(
        word: String,
        pattern_perm: &Vec<String>,
        pattern_stack: &pattern::PatternStack,
        words: &Vec<String>,
    ) -> Self {
        let mut sum_probability: f64 = 0.0;
        let mut sum_occurance = 0;

        for pattern_str in pattern_perm.iter() {
            let pattern =
                pattern::PatternLine::try_from((word.as_str(), pattern_str.as_str())).unwrap();

            let mut pattern_stack = pattern_stack.clone();
            pattern_stack.progress(pattern).unwrap();

            let possible_word_count = pattern_stack.possible_word_count(words) as f64;
            let probability = possible_word_count as f64 / words.len() as f64;

            if probability.abs() > f64::EPSILON {
                sum_occurance += 1;
            }

            sum_probability += probability;
        }

        // let probability = sum_probability / pattern_perm.len() as f64;
        let probability = sum_probability / sum_occurance as f64;

        let entropy = -(probability.log2());

        // println!("{}: {:.3}", word, bit);

        Self { word, entropy }

        // ========================
    }

    pub fn word(&self) -> &str {
        self.word.as_str()
    }

    pub fn entropy(&self) -> f64 {
        self.entropy
    }
}

// ===============================================

#[derive(Debug)]
pub struct EntropySet {
    entropy_set: Vec<EntropyUnit>,
}

impl EntropySet {
    fn new(
        pattern_perm: &Vec<String>,
        pattern_stack: &pattern::stack::PatternStack,
        words: &Vec<String>,
    ) -> Self {
        use rayon::prelude::*;
        use std::sync::Arc;
        use std::sync::Mutex;

        let entropy_set = Vec::<EntropyUnit>::with_capacity(words.len());
        let entropy_set = Arc::new(Mutex::new(entropy_set));

        let count = Arc::new(Mutex::new(0));

        words.par_iter().for_each(|word| {
            let entropy_unit =
                EntropyUnit::new(word.to_owned(), pattern_perm, pattern_stack, words);
            (*entropy_set.lock().unwrap()).push(entropy_unit);

            *count.lock().unwrap() += 1;
            cli::print_calculating(word.as_str(), *count.lock().unwrap(), words.len());
        });

        let entropy_set = entropy_set.lock().unwrap().to_owned();

        Self { entropy_set }
    }

    fn map_words(&self) -> Vec<String> {
        self.entropy_set
            .iter()
            .map(|entropy| entropy.word.to_owned())
            .collect::<Vec<String>>()
    }

    pub fn entropy_ranking(&self, n: usize) -> Vec<EntropyUnit> {
        let mut v = self.entropy_set.to_owned();
        // v.sort_by(|a, b| b.bit.partial_cmp(&a.bit).unwrap());
        v.sort_by(|a, b| (b.entropy).partial_cmp(&a.entropy).unwrap());
        v.into_iter().take(n).collect::<Vec<EntropyUnit>>()
        // v.into_iter().collect::<Vec<EntropyUnit>>()
    }
}

// ===============================================

#[derive(Debug)]
pub struct EntropyStack {
    entropy_stack: Vec<EntropySet>,
}

impl EntropyStack {
    pub fn new() -> Self {
        let entropy_stack = Vec::<EntropySet>::new();
        Self { entropy_stack }
    }

    pub fn current_entropy_count(&self) -> usize {
        self.entropy_stack.last().unwrap().entropy_set.len()
    }

    pub fn current_entropy_set(&self) -> &EntropySet {
        self.entropy_stack.last().unwrap()
    }

    pub fn current_words(&self) -> Vec<String> {
        self.entropy_stack.last().unwrap().map_words()
    }

    pub fn progress(
        &mut self,
        pattern_perm: &Vec<String>,
        pattern_stack: &pattern::stack::PatternStack,
        words: &Vec<String>,
    ) {
        let entropy_set = EntropySet::new(pattern_perm, pattern_stack, words);

        self.entropy_stack.push(entropy_set);
    }

    pub fn revert(&mut self) -> Option<EntropySet> {
        self.entropy_stack.pop()
    }
}

// ===============================================
