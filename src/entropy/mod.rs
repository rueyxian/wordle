// ========================
// ===============================================
// ===============================================================================================

use super::pattern;
use super::permutation;

// ===============================================

#[derive(Debug)]
struct WordEntropy<'a> {
    word: &'a str,
    bit: f64,
}

impl<'a> WordEntropy<'a> {
    fn new(
        word: &'a str,
        pattern_perm: &Vec<String>,
        curr_pattern_stack: &mut pattern::PatternStack,
        // curr_words: &Vec<String>,
        curr_words: &Vec<&str>,
    ) -> Self {
        let mut sum_probability: f64 = 0.0;

        let mut occurance = 0;

        for pattern_str in pattern_perm.iter() {
            let pattern = pattern::PatternLine::try_from((word, pattern_str.as_str())).unwrap();

            // let mut possible_pattern = curr_pattern_stack;
            // possible_pattern.add_pattern_line(pattern).unwrap();
            //
            // let probability =
            //     possible_pattern.possible_word_count(curr_words) as f64 / curr_words.len() as f64;

            // curr_pattern_stack.add_pattern_line(pattern).unwrap();
            let probability =
                curr_pattern_stack.possible_word_count(curr_words) as f64 / curr_words.len() as f64;
            // curr_pattern_stack.remove_last_pattern_line();

            if probability.abs() < f64::EPSILON {
                occurance += 1;
            }

            sum_probability += probability;
        }

        let probability = sum_probability / occurance as f64;
        let bit = -(probability.log2());

        Self { word, bit }
    }
}

// ===============================================

pub struct Universe<'a> {
    curr_words: Vec<&'a str>,
    unit_count: usize,
    curr_pattern_stack: pattern::stack::PatternStack,

    pattern_perms: Vec<String>,
}

impl<'a> Universe<'a> {
    pub fn new(unit_count: usize, words: Vec<&'a str>) -> Self {
        let mut pattern_perm = Vec::<String>::new();
        let iter = permutation::Permutations::new(&["#", "?", "!"], unit_count);

        iter.for_each(|perm| pattern_perm.push(perm.join("")));

        Self {
            curr_words: words,
            unit_count,
            curr_pattern_stack: pattern::stack::PatternStack::new(unit_count),
            pattern_perms: pattern_perm,
        }
    }

    fn generate_word_entropies(&self) -> Vec<WordEntropy> {
        let mut word_entropies = Vec::with_capacity(self.curr_words.len());
        let mut curr_pattern_stack = self.curr_pattern_stack.clone();

        for word in &self.curr_words {
            let word_entropy = WordEntropy::new(
                word,
                &self.pattern_perms,
                &mut curr_pattern_stack,
                &self.curr_words,
            );
            word_entropies.push(word_entropy);
        }
        word_entropies
    }

    // TODO: TEMPORARY!!!!!
    fn generate_possible_pattern_lines(&self) -> Vec<(&str, Vec<pattern::PatternLine>)> {
        let mut lines =
            Vec::<(&str, Vec<pattern::PatternLine>)>::with_capacity(self.curr_words.len());

        for &word in &self.curr_words {
            let mut possible = Vec::<pattern::PatternLine>::with_capacity(self.pattern_perms.len());

            for perm in &self.pattern_perms {
                let pattern_line = pattern::PatternLine::try_from((word, perm.as_str())).unwrap();
                possible.push(pattern_line);
            }
            lines.push((word, possible));
        }
        lines
    }
}

// ===============================================

#[cfg(test)]
mod tests {

    use super::*;
    use crate::word_source::WordSource;

    // ===============================================

    // #[ignore]
    #[test]
    fn generate_entropies() {
        let wv = WordSource::new();
        let words = wv.read_vault().unwrap();
        let words = words
            .iter()
            .map(|word| word.as_str())
            .collect::<Vec<&str>>();

        // let words = vec![
        //     "okays", "loons", "howdy", "likes", "frump", "lures", "hiked", "laird", "sinew",
        //     "icily", "ewers", "obeys", "dirty", "pesky", "agony", "keels", "apart", "gluey",
        //     "resin", "skunk", "trait", "tests", "picks", "smirk", "stray", "begat", "fools",
        //     "corps", "mange", "odour", "ruins", "slurs", "reins", "inked", "rucks", "voile",
        //     "trips", "honor", "pukka", "yuppy", "bided", "beard", "inset", "alone", "racer",
        //     "bluff", "cleat", "swats", "ashen", "brats", "sized", "sours", "greed", "flubs",
        //     "speck", "nooky", "climb", "chows", "shoos", "wrote", "dicey", "equal", "torts",
        //     "their", "stood", "posse", "gusto", "added", "reedy", "bulgy", "evens", "savvy",
        //     "laxly", "colds", "crest", "knees", "mutts", "desks", "amity", "mazes", "pubic",
        //     "gawps", "bijou", "burst", "rearm", "bhaji", "betel", "audio", "blond", "draws",
        //     "yards", "johns", "civet", "reams", "excel", "yawns", "hokum", "posed", "hands",
        //     "julep", "puffs", "ledge", "stiff", "width", "quail", "avail", "irked", "pates",
        //     "ailed", "doffs", "fared", "faced", "nasty", "harps", "slobs", "salts", "start",
        //     "omits", "karma", "stirs", "boxes", "cacti", "quell", "moose", "tubby", "kilts",
        //     "cater", "lovey", "loose", "sorry", "surge", "dizzy", "jammy", "soppy", "slink",
        //     "write", "bonks", "asset", "brave", "capon", "medic", "prism", "quick", "tires",
        //     "fists", "velar", "scour", "upped", "abler", "raids", "rough", "scaly", "shale",
        //     "human", "chuck", "lowly", "lurks", "azure", "elder", "fatso", "first", "opals",
        //     "sofas", "idiom", "sieve", "taxes", "gulps", "crazy", "never", "abode", "bidet",
        //     "hedge", "views", "winos", "snail", "mocha", "woman", "yarns", "group", "hawed",
        //     "stark", "buoys", "colts", "mania", "shove", "recto", "irons", "axing", "hallo",
        //     "wacko", "lathe", "safes", "fazes", "nines", "there", "cable", "honky", "wafts",
        //     "touts", "sends", "shoes", "cedes", "glitz", "quiff", "ruble", "booby", "cedar",
        //     "clans", "flyby", "lulls", "spear", "sweet", "tramp", "crags", "krill", "lover",
        //     "purse", "pouts", "imbed", "muses", "shiny", "sifts", "lobed", "clams", "colic",
        //     "tonal", "break", "canny", "games", "chore", "livid", "ovary", "tails", "scout",
        //     "caves", "north", "pause", "shone", "teats", "amass", "egret", "train", "exalt",
        //     "baits", "stein", "zippy", "flame", "odors", "sicko", "booze", "feuds", "scoot",
        //     "biffs", "dingo", "fluff", "hefty", "later", "madly", "scrub", "foams", "sniff",
        //     "midst", "these", "baccy", "darts", "enjoy", "moved", "glass", "panel", "scrum",
        //     "inter", "whale", "catch", "micks", "polls", "drawn", "impel", "forge", "frail",
        //     "mould", "huffy", "yogic", "meres", "pivot", "byway", "broth", "oiled", "allot",
        //     "adorn", "focal", "clods", "necks", "swizz", "hubby", "chest", "oozes", "shelf",
        //     "mammy", "rerun", "pizza", "surfs", "weeps", "coded", "pages", "dhows", "dries",
        //     "pokey", "beads", "pixel", "ranee", "gales", "manor", "scrap", "spars", "toads",
        //     "venom", "gizmo", "panic", "remix", "smogs", "humid",
        // ];

        println!("words len: {}", words.len());

        let universe = Universe::new(5, words);

        let entropies = universe.generate_word_entropies();

        for entropy in entropies {
            println!("word: {} | entropy: {}", entropy.word, entropy.bit);
        }
    }

    // ===============================================
    #[ignore]
    #[test]
    fn generate_pattern_line() {
        let wv = WordSource::new();
        let words = wv.read_vault().unwrap();
        let words = words
            .iter()
            .map(|word| word.as_str())
            .collect::<Vec<&str>>();

        let universe = Universe::new(5, words);

        let possible_patterns = universe.generate_possible_pattern_lines();

        println!("{:?}", possible_patterns);
    }

    // ===============================================

    #[ignore]
    #[test]
    fn temp() {
        let wv = WordSource::new();
        let words = wv.read_vault().unwrap();

        // let set =  std::collections::HashSet::new();
        let set = words
            .into_iter()
            .collect::<std::collections::HashSet<String>>();

        for word in set.iter() {
            println!("{}", word);
        }
    }

    // ===============================================

    // #[test]
    // fn calculate_entropy() {
    //     let wv = WordVault::new();
    //     let words = wv.read_vault().unwrap();
    //
    //     let mut universe = Universe::new(5);
    //
    //     let word = "corgi";
    //
    //     // ========================
    //
    //     let mut pattern_perm = Vec::<String>::new();
    //     let iter = Permutations::new(&["#", "?", "!"], 5);
    //     iter.for_each(|perm| pattern_perm.push(perm.join("")));
    //
    //     // ========================
    //
    //     let mut possible_patterns = Vec::new();
    //
    //     for pattern_str in pattern_perm {
    //         let pattern = Pattern::try_from((word, pattern_str.as_str())).unwrap();
    //
    //         possible_patterns.push(pattern);
    //     }
    //
    //     println!("{:?}", possible_patterns);
    // }
}
