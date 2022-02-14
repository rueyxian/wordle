// ========================
// ===============================================
// ===============================================================================================

use super::entropy;
use super::pattern;
use super::permutation;

// ===============================================

pub enum Error {
    Revert,
}

// ===============================================

#[derive(Debug)]
pub struct Universe {
    // unit_count: usize,
    entropy_stack: entropy::EntropyStack,
    pattern_stack: pattern::stack::PatternStack,

    pattern_perms: Vec<String>,
}

impl Universe {
    pub fn new(unit_count: usize, words: Vec<String>) -> Self {
        let mut pattern_perm = Vec::<String>::new();
        let perm_iter = permutation::Permutations::new(&["#", "?", "!"], unit_count);
        perm_iter.for_each(|perm| pattern_perm.push(perm.join("")));

        let pattern_stack = pattern::stack::PatternStack::new(unit_count);

        let mut entropy_stack = entropy::EntropyStack::new();
        entropy_stack.progress(&pattern_perm, &pattern_stack, &words);

        Self {
            // unit_count,
            entropy_stack,
            pattern_stack,
            pattern_perms: pattern_perm,
        }
    }

    // ========================

    pub fn progress(&mut self, pattern_line: pattern::stack::PatternLine) {
        self.pattern_stack.progress(pattern_line).unwrap();

        let words = self.entropy_stack.current_words();
        let words = self.pattern_stack.possible_words(&words);

        self.entropy_stack
            .progress(&self.pattern_perms, &self.pattern_stack, &words)
    }

    pub fn revert(&mut self) -> Result<(), Error> {
        (self.pattern_stack.revert().is_some() && self.entropy_stack.revert().is_some())
            .then(|| ())
            .ok_or(Error::Revert)
    }

    // ========================
    pub fn posibility(&self) -> usize {
        self.entropy_stack.current_entropy_count()
    }

    pub fn entropy_ranking(&self, n: usize) -> Vec<entropy::EntropyUnit> {
        self.entropy_stack.current_entropy_set().entropy_ranking(n)
    }

    pub fn pattern_stack(&self) -> &pattern::PatternStack {
        &self.pattern_stack
    }
}

// ===============================================

#[cfg(test)]
mod tests {

    // ===============================================

    #[ignore]
    #[test]
    fn basic() {}
}
