use crate::cli;

use crate::pattern;

use crate::entropy;

#[derive(Debug)]
pub struct EntropySet {
    entropy_set: Vec<entropy::EntropyUnit>,
}

impl EntropySet {
    pub fn new(
        pattern_perm: &Vec<String>,
        pattern_stack: &pattern::PatternStack,
        words: &Vec<String>,
    ) -> Self {
        use rayon::prelude::*;
        use std::sync::Arc;
        use std::sync::Mutex;

        let entropy_set = Vec::<entropy::EntropyUnit>::with_capacity(words.len());
        let entropy_set = Arc::new(Mutex::new(entropy_set));

        let count = Arc::new(Mutex::new(0));

        words.par_iter().for_each(|word| {
            let entropy_unit =
                entropy::EntropyUnit::new(word.to_owned(), pattern_perm, pattern_stack, words);
            (*entropy_set.lock().unwrap()).push(entropy_unit);

            *count.lock().unwrap() += 1;
            cli::print_calculating(word.as_str(), *count.lock().unwrap(), words.len());
        });

        let entropy_set = entropy_set.lock().unwrap().to_owned();

        Self { entropy_set }
    }

    pub fn set_size(&self) -> usize {
        self.entropy_set.len()
    }

    pub fn map_words(&self) -> Vec<String> {
        self.entropy_set
            .iter()
            .map(|entropy| entropy.word().to_owned())
            .collect::<Vec<String>>()
    }

    pub fn entropy_ranking(&self, n: usize) -> Vec<entropy::EntropyUnit> {
        let mut v = self.entropy_set.to_owned();
        v.sort_by(|a, b| (b.entropy()).partial_cmp(&a.entropy()).unwrap());
        v.into_iter().take(n).collect::<Vec<entropy::EntropyUnit>>()
    }
}
