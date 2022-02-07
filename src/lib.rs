// ========================
// ===============================================
// ===============================================================================================

pub mod vault;

pub mod search;

// ===============================================

use std::collections::HashSet;
use std::error;

use crate::search::Search;
use crate::vault::WordVault;

// ===============================================

pub fn search(input: &str) -> Result<(HashSet<String>, bool), Box<dyn error::Error>> {
    let wv = WordVault::new();
    let set = wv.read_vault().unwrap();

    let search = Search::parse(input)?;

    Ok(search.search(set))
}

// ===============================================

#[cfg(test)]
mod tests {

    use crate::search::Search;
    use crate::vault::WordVault;

    #[ignore]
    #[test]
    fn parse_and_search() {
        let wv = WordVault::new();
        let set = wv.read_vault().unwrap();

        let search = Search::parse("aes| # o # ?p # |").unwrap();

        let (result, _) = search.search(set);

        println!("{:?}", result);
    }

    #[ignore]
    #[test]
    fn isogram() {
        let wv = WordVault::new();
        let set = wv.read_vault().unwrap();

        let search = Search::parse("a| # # # # # |i").unwrap();

        let (result, _) = search.search(set);

        println!("{:?}", result);
    }
}
