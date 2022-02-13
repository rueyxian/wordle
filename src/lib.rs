// ========================
// ===============================================
// ===============================================================================================

pub mod cli;

pub mod word_pool;

pub mod permutation;

// pub mod search;

pub mod pattern;

pub mod entropy;

// ===============================================

use std::error;

// ===============================================

pub fn search(input: &str) -> Result<Vec<String>, Box<dyn error::Error>> {
    let wv = word_pool::WordPool::new();
    let word_set = wv.read_pool().unwrap();
    let word_set = word_set
        .iter()
        .map(|word| word.as_str())
        .collect::<Vec<&str>>();

    // let search = Search::parse(input)?;

    // let universe = entropy::Universe::new(5, word_set);
    // let possible_words = universe.poss

    let mut pattern_stack = pattern::PatternStack::new(5);
    let pattern_line = pattern::PatternLine::try_from(input).unwrap();

    pattern_stack.add_pattern_line(pattern_line).unwrap();

    let possible_words = pattern_stack
        .possible_words(&word_set)
        .iter()
        .map(|&word| word.to_owned())
        .collect::<Vec<String>>();

    // let possible_words = pattern_stack
    //     .possible_words(&word_set)
    //     .iter()
    //     .map(|&word| {
    //         let x = word.to_owned();
    //         x
    //     })
    //     .collect::<Vec<String>>();

    Ok(possible_words)
}

// ===============================================

#[cfg(test)]
mod tests {

    // use crate::search::Search;
    use crate::word_pool::WordPool;

    #[ignore]
    #[test]
    fn parse_and_search() {
        // let wv = WordSource::new();
        // let set = wv.read_vault().unwrap();
        //
        // let search = Search::parse("aes| # o # ?p # |").unwrap();
        //
        // let (result, _) = search.search(set);
        //
        // println!("{:?}", result);
    }
}
