// ========================
// ===============================================
// ===============================================================================================

use std::sync::Arc;
use std::sync::Mutex;

use super::pattern;
use super::permutation;

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
pub struct WordEntropy<'a> {
    word: &'a str,
    bit: f32,
}

impl<'a> WordEntropy<'a> {
    fn new(
        word: &'a str,
        pattern_perm: &Vec<String>,
        curr_pattern_stack: &mut pattern::PatternStack,
        // curr_words: &Vec<String>,
        curr_words: &Vec<String>,
    ) -> Self {
        // use rayon::prelude::*;
        // let curr_pattern_stack = Mutex::new(curr_pattern_stack);
        // let sum_probability = Mutex::new(0.0_f32);
        // let occurance = Mutex::new(0_u64);
        //
        // pattern_perm.par_iter().for_each(|pattern_str| {
        //     let pattern =
        //         pattern::PatternLine::try_from((word.as_str(), pattern_str.as_str())).unwrap();
        //
        //     (*curr_pattern_stack.lock().unwrap())
        //         .add_pattern_line(pattern)
        //         .unwrap();
        //
        //     let probability = (*curr_pattern_stack.lock().unwrap()).possible_word_count(curr_words)
        //         as f32
        //         / curr_words.len() as f32;
        //
        //     (*curr_pattern_stack.lock().unwrap()).remove_last_pattern_line();
        //
        //     if probability.abs() < f32::EPSILON {
        //         *occurance.lock().unwrap() += 1;
        //     }
        //
        //     *sum_probability.lock().unwrap() += probability;
        // });
        //
        // let probability = *sum_probability.lock().unwrap() / *occurance.lock().unwrap() as f32;
        //
        // let bit = -(probability.log2());
        //
        // println!("{}: {} bit", word, bit);
        //
        // Self { word, bit }

        // ========================

        let mut sum_probability: f32 = 0.0;
        let mut occurance = 0;

        for pattern_str in pattern_perm.iter() {
            let pattern = pattern::PatternLine::try_from((word, pattern_str.as_str())).unwrap();

            curr_pattern_stack.add_pattern_line(pattern).unwrap();

            let probability =
                curr_pattern_stack.possible_word_count(curr_words) as f32 / curr_words.len() as f32;

            curr_pattern_stack.remove_last_pattern_line();

            if probability.abs() < f32::EPSILON {
                occurance += 1;
            }

            sum_probability += probability;
        }

        let probability = sum_probability / occurance as f32;

        let bit = -(probability.log2());

        println!("{}: {:.3}", word, bit);

        Self { word, bit }

        // ========================
    }
}

// ===============================================

// pub struct Words<'a> {
//     words: Vec<WordEntropy<'a>>,
// }
//
// impl<'a> Words<'a> {
//
// }

// ===============================================

pub struct Universe<'a> {
    curr_words: Vec<&'a str>,

    // word_entropies: Vec<WordEntropy>,
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

    // ========================

    // pub fn curr_pattern_stack(&self) -> pattern::stack::PatternStack {
    //     self.curr_pattern_stack
    // }

    // ========================

    // pub fn add_pattern_line(
    //     &mut self,
    //     pattern_line: pattern::stack::PatternLine,
    // ) -> Result<(), pattern::stack::Error> {
    //     self.curr_pattern_stack.add_pattern_line(pattern_line)
    // }

    // ========================

    // pub fn calculate_entropies(&self) {
    //     use rayon::prelude::*;
    //     let word_entropies = Mutex::new(Vec::<WordEntropy>::with_capacity(self.curr_words.len()));
    //
    //     let curr_words = self
    //         .curr_words
    //         .iter()
    //         .map(|&word| word.to_owned())
    //         .collect::<Vec<String>>();
    //
    //     self.curr_words.par_iter().for_each(|word| {
    //         let word_entropy = WordEntropy::new(
    //             word,
    //             &self.pattern_perms,
    //             &mut self.curr_pattern_stack.clone(),
    //             // &self.curr_words,
    //             &curr_words,
    //         );
    //         word_entropies.lock().unwrap().push(word_entropy);
    //     });
    //     let x = word_entropies.lock().unwrap().to_owned();
    //     x
    // }

    // ========================

    pub fn generate_word_entropies(&self) -> Vec<WordEntropy> {
        use rayon::prelude::*;
        let word_entropies = Mutex::new(Vec::<WordEntropy>::with_capacity(self.curr_words.len()));

        let curr_words = self
            .curr_words
            .iter()
            .map(|&word| word.to_owned())
            .collect::<Vec<String>>();

        self.curr_words.par_iter().for_each(|word| {
            let word_entropy = WordEntropy::new(
                word,
                &self.pattern_perms,
                &mut self.curr_pattern_stack.clone(),
                // &self.curr_words,
                &curr_words,
            );
            word_entropies.lock().unwrap().push(word_entropy);
        });
        let x = word_entropies.lock().unwrap().to_owned();
        x
    }

    // ========================

    // pub fn generate_word_entropies(&self) -> Vec<WordEntropy> {
    //     // let threadpool = threadpool::Builder::new().num_threads(16).build();
    //
    //     let worker_num = 8;
    //
    //     let threadpool = threadpool::ThreadPool::new(worker_num);
    //
    //     // let mut handlers = Vec::<thread::JoinHandle<()>>::new();
    //     let word_entropies = Arc::new(Mutex::new(Vec::<WordEntropy>::with_capacity(
    //         self.curr_words.len(),
    //     )));
    //
    //     let pattern_perms = Arc::new(Mutex::new(self.pattern_perms.clone()));
    //     let curr_pattern_stack = Arc::new(Mutex::new(self.curr_pattern_stack.clone()));
    //
    //     let curr_words = self
    //         .curr_words
    //         .iter()
    //         .map(|&word| word.to_owned())
    //         .collect::<Vec<String>>();
    //     let curr_words = Arc::new(Mutex::new(curr_words));
    //
    //
    //
    //     // TODO
    //     threadpool.execute(move || {
    //         //
    //     });
    //
    //
    //
    //     // for &word in self.curr_words.iter() {
    //     //     let word_entropies = Arc::clone(&word_entropies);
    //     //
    //     //     let word = word.to_owned();
    //     //     let pattern_perms = Arc::clone(&pattern_perms);
    //     //     let curr_pattern_stack = Arc::clone(&curr_pattern_stack);
    //     //     let curr_words = Arc::clone(&curr_words);
    //     //
    //     //     threadpool.execute(move || {
    //     //         let word_entropy = WordEntropy::new(
    //     //             word,
    //     //             &*pattern_perms.lock().unwrap(),
    //     //             &mut *curr_pattern_stack.lock().unwrap(),
    //     //             &*curr_words.lock().unwrap(),
    //     //         );
    //     //         word_entropies.lock().unwrap().push(word_entropy);
    //     //     });
    //     //
    //     //     // handlers.push(handler)
    //     // }
    //     threadpool.join();
    //     let x = word_entropies.lock().unwrap().to_owned();
    //     x
    // }

    // ========================

    // pub fn generate_word_entropies(&self) -> Vec<WordEntropy> {
    //     // let threadpool = threadpool::Builder::new().num_threads(16).build();
    //     let threadpool = threadpool::ThreadPool::new(2000);
    //
    //     // let mut handlers = Vec::<thread::JoinHandle<()>>::new();
    //     let word_entropies = Arc::new(Mutex::new(Vec::<WordEntropy>::with_capacity(
    //         self.curr_words.len(),
    //     )));
    //
    //     let pattern_perms = Arc::new(Mutex::new(self.pattern_perms.clone()));
    //     let curr_pattern_stack = Arc::new(Mutex::new(self.curr_pattern_stack.clone()));
    //
    //     let curr_words = self
    //         .curr_words
    //         .iter()
    //         .map(|&word| word.to_owned())
    //         .collect::<Vec<String>>();
    //     let curr_words = Arc::new(Mutex::new(curr_words));
    //
    //     for &word in self.curr_words.iter() {
    //         let word_entropies = Arc::clone(&word_entropies);
    //
    //         let word = word.to_owned();
    //         let pattern_perms = Arc::clone(&pattern_perms);
    //         let curr_pattern_stack = Arc::clone(&curr_pattern_stack);
    //         let curr_words = Arc::clone(&curr_words);
    //
    //         threadpool.execute(move || {
    //             let word_entropy = WordEntropy::new(
    //                 word,
    //                 &*pattern_perms.lock().unwrap(),
    //                 &mut *curr_pattern_stack.lock().unwrap(),
    //                 &*curr_words.lock().unwrap(),
    //             );
    //             word_entropies.lock().unwrap().push(word_entropy);
    //         });
    //
    //         // handlers.push(handler)
    //     }
    //     threadpool.join();
    //     let x = word_entropies.lock().unwrap().to_owned();
    //     x
    // }

    // ========================

    // pub fn generate_word_entropies(&self) -> Vec<WordEntropy> {
    //     use threadpool;
    //
    //     let tpool = threadpool::Builder::new()
    //         .num_threads(8)
    //         .thread_stack_size(8_000_000)
    //         .build();
    //
    //     let mut handlers = Vec::<thread::JoinHandle<()>>::new();
    //     let word_entropies = Arc::new(Mutex::new(Vec::<WordEntropy>::with_capacity(
    //         self.curr_words.len(),
    //     )));
    //
    //     let pattern_perms = Arc::new(Mutex::new(self.pattern_perms.clone()));
    //     let curr_pattern_stack = Arc::new(Mutex::new(self.curr_pattern_stack.clone()));
    //
    //     let curr_words = self
    //         .curr_words
    //         .iter()
    //         .map(|&word| word.to_owned())
    //         .collect::<Vec<String>>();
    //     let curr_words = Arc::new(Mutex::new(curr_words));
    //
    //     for &word in self.curr_words.iter() {
    //         let word_entropies = Arc::clone(&word_entropies);
    //
    //         let word = word.to_owned();
    //         let pattern_perms = Arc::clone(&pattern_perms);
    //         let curr_pattern_stack = Arc::clone(&curr_pattern_stack);
    //         let curr_words = Arc::clone(&curr_words);
    //
    //         let handler = thread::spawn(move || {
    //             let word_entropy = WordEntropy::new(
    //                 word,
    //                 &*pattern_perms.lock().unwrap(),
    //                 &mut *curr_pattern_stack.lock().unwrap(),
    //                 &*curr_words.lock().unwrap(),
    //             );
    //             word_entropies.lock().unwrap().push(word_entropy);
    //         });
    //         handlers.push(handler)
    //     }
    //
    //     for handle in handlers {
    //         handle.join().unwrap();
    //     }
    //
    //     let x = word_entropies.lock().unwrap().to_owned();
    //     x
    // }

    // ========================

    // pub fn generate_word_entropies(&self) -> Vec<WordEntropy> {
    //     let mut word_entropies = Vec::with_capacity(self.curr_words.len());
    //     let curr_words = self
    //         .curr_words
    //         .iter()
    //         .map(|&word| word.to_owned())
    //         .collect::<Vec<String>>();
    //     for word in self.curr_words.iter() {
    //         let word_entropy = WordEntropy::new(
    //             word.to_string(),
    //             &self.pattern_perms,
    //             &mut self.curr_pattern_stack.clone(),
    //             &curr_words,
    //         );
    //         word_entropies.push(word_entropy);
    //     }
    //     word_entropies
    // }

    // ========================

    // fn generate_word_entropies(&self) -> Vec<WordEntropy> {
    //     let mut word_entropies = Vec::with_capacity(self.curr_words.len());
    //     for word in self.curr_words.iter() {
    //         let word_entropy = WordEntropy::new(
    //             word,
    //             &self.pattern_perms,
    //             &mut self.curr_pattern_stack.clone(),
    //             &self.curr_words,
    //         );
    //         word_entropies.push(word_entropy);
    //     }
    //     word_entropies
    // }

    // ========================
}

// ===============================================

#[cfg(test)]
mod tests {

    use super::*;
    use crate::word_pool::WordPool;

    // ===============================================

    // #[ignore]
    #[test]
    fn generate_entropies() {
        // let wv = WordPool::new();
        // let words = wv.read_pool().unwrap();
        // let words = words
        //     .iter()
        //     .map(|word| word.as_str())
        //     .collect::<Vec<&str>>();

        let words = vec![
            "okays", "loons", "howdy", "likes", "frump", "lures", "hiked", "laird", "sinew",
            "icily", "ewers", "obeys", "dirty", "pesky", "agony", "keels", "apart", "gluey",
            "resin", "skunk", "trait", "tests", "picks", "smirk", "stray", "begat", "fools",
            "corps", "mange", "odour", "ruins", "slurs", "reins", "inked", "rucks", "voile",
            "trips", "honor", "pukka", "yuppy", "bided", "beard", "inset", "alone", "racer",
            "bluff", "cleat", "swats", "ashen", "brats", "sized", "sours", "greed",
            "flubs",
            // "speck", "nooky", "climb", "chows", "shoos", "wrote", "dicey", "equal", "torts",
            // "their", "stood", "posse", "gusto", "added", "reedy", "bulgy", "evens", "savvy",
            // "laxly", "colds", "crest", "knees", "mutts", "desks", "amity", "mazes", "pubic",
            // "gawps", "bijou", "burst", "rearm", "bhaji", "betel", "audio", "blond", "draws",
            // "yards", "johns", "civet", "reams", "excel", "yawns", "hokum", "posed", "hands",
            // "julep", "puffs", "ledge", "stiff", "width", "quail", "avail", "irked", "pates",
            // "ailed", "doffs", "fared", "faced", "nasty", "harps", "slobs", "salts", "start",
            // "omits", "karma", "stirs", "boxes", "cacti", "quell", "moose", "tubby", "kilts",
            // "cater", "lovey", "loose", "sorry", "surge", "dizzy", "jammy", "soppy", "slink",
            // "write", "bonks", "asset", "brave", "capon", "medic", "prism", "quick", "tires",
            // "fists", "velar", "scour", "upped", "abler", "raids", "rough", "scaly", "shale",
            // "human", "chuck", "lowly", "lurks", "azure", "elder", "fatso", "first", "opals",
            // "sofas", "idiom", "sieve", "taxes", "gulps", "crazy", "never", "abode", "bidet",
            // "hedge", "views", "winos", "snail", "mocha", "woman", "yarns", "group", "hawed",
            // "stark", "buoys", "colts", "mania", "shove", "recto", "irons", "axing", "hallo",
            // "wacko", "lathe", "safes", "fazes", "nines", "there", "cable", "honky", "wafts",
            // "touts", "sends", "shoes", "cedes", "glitz", "quiff", "ruble", "booby", "cedar",
            // "clans", "flyby", "lulls", "spear", "sweet", "tramp", "crags", "krill", "lover",
            // "purse", "pouts", "imbed", "muses", "shiny", "sifts", "lobed", "clams", "colic",
            // "tonal", "break", "canny", "games", "chore", "livid", "ovary", "tails", "scout",
            // "caves", "north", "pause", "shone", "teats", "amass", "egret", "train", "exalt",
            // "baits", "stein", "zippy", "flame", "odors", "sicko", "booze", "feuds", "scoot",
            // "biffs", "dingo", "fluff", "hefty", "later", "madly", "scrub", "foams", "sniff",
            // "midst", "these", "baccy", "darts", "enjoy", "moved", "glass", "panel", "scrum",
            // "inter", "whale", "catch", "micks", "polls", "drawn", "impel", "forge", "frail",
            // "mould", "huffy", "yogic", "meres", "pivot", "byway", "broth", "oiled", "allot",
            // "adorn", "focal", "clods", "necks", "swizz", "hubby", "chest", "oozes", "shelf",
            // "mammy", "rerun", "pizza", "surfs", "weeps", "coded", "pages", "dhows", "dries",
            // "pokey", "beads", "pixel", "ranee", "gales", "manor", "scrap", "spars", "toads",
            // "venom", "gizmo", "panic", "remix", "smogs", "humid",
        ];

        println!("words len: {}", words.len());

        let universe = Universe::new(5, words);

        let entropies = universe.generate_word_entropies();

        for entropy in entropies {
            println!("word: {} | entropy: {}", entropy.word, entropy.bit);
        }
    }

    // ===============================================
    // #[ignore]
    // #[test]
    // fn generate_pattern_line() {
    //     let wv = WordSource::new();
    //     let words = wv.read_vault().unwrap();
    //     let words = words
    //         .iter()
    //         .map(|word| word.as_str())
    //         .collect::<Vec<&str>>();
    //
    //     let universe = Universe::new(5, words);
    //
    //     let possible_patterns = universe.generate_possible_pattern_lines();
    //
    //     println!("{:?}", possible_patterns);
    // }

    // ===============================================

    // #[ignore]
    // #[test]
    // fn rayon_test() {
    //     use rayon::prelude::*;
    //
    //     let s = ['a', 'b', 'c', 'd', 'e']
    //         .par_iter()
    //         .fold(
    //             || String::new(),
    //             |mut s: String, c: &char| {
    //                 s.push(*c);
    //                 s
    //             },
    //         )
    //         .reduce(
    //             || String::new(),
    //             |mut a: String, b: String| {
    //                 a.push_str(&b);
    //                 a
    //             },
    //         );
    // }

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
