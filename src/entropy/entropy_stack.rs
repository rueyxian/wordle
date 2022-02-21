use crate::entropy;

use crate::pattern;

#[derive(Debug)]
pub struct EntropyStack {
    entropy_stack: Vec<entropy::EntropySet>,
}

impl EntropyStack {
    pub fn new() -> Self {
        let entropy_stack = Vec::<entropy::EntropySet>::new();
        Self { entropy_stack }
    }

    pub fn current_entropy_count(&self) -> usize {
        self.entropy_stack.last().unwrap().set_size()
    }

    pub fn current_entropy_set(&self) -> &entropy::EntropySet {
        self.entropy_stack.last().unwrap()
    }

    pub fn current_words(&self) -> Vec<String> {
        self.entropy_stack.last().unwrap().map_words()
    }

    pub fn progress(
        &mut self,
        pattern_perm: &Vec<String>,
        pattern_stack: &pattern::PatternStack,
        words: &Vec<String>,
    ) {
        let entropy_set = entropy::EntropySet::new(pattern_perm, pattern_stack, words);

        self.entropy_stack.push(entropy_set);
    }

    pub fn revert(&mut self) -> Option<entropy::EntropySet> {
        self.entropy_stack.pop()
    }
}
