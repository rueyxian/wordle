use crate::pattern;

#[derive(Debug, Clone)]
pub struct EntropyUnit {
    word: String,
    entropy: f64,
}

impl EntropyUnit {
    pub fn new(
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

        let probability = sum_probability / sum_occurance as f64;
        let mut entropy = -(probability.log2());

        if entropy.abs() < f64::EPSILON {
            // If it is negatively close to zero, set it to zero.
            // It won't really affect the outcome if we don't do that,
            // just that `-0.00000..` is not so eye pleasing.
            entropy = 0.0;
        }

        Self { word, entropy }
    }

    pub fn word(&self) -> &str {
        self.word.as_str()
    }

    pub fn entropy(&self) -> f64 {
        self.entropy
    }
}
