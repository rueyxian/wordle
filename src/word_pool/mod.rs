// ========================
// ===============================================
// ===============================================================================================

use std::io::BufRead;

// ===============================================

#[derive(Debug)]
pub enum Error {
    OpenError(std::io::Error),
    ReadError(std::io::Error),
}

pub struct WordPool {
    path: String,
}

impl WordPool {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap();
        Self {
            // path: format!("{}/.wordle/word_pool", home),
            path: format!("{}/.wordle/original_pool", home),
        }
    }

    pub fn read_pool(&self) -> Result<Vec<String>, Error> {
        let path = std::path::Path::new(self.path.as_str());
        let file = std::fs::File::open(path).map_err(|e| Error::OpenError(e))?;

        let buf_reader = std::io::BufReader::new(file);

        // TODO: return error instead of unwrap()
        let words = buf_reader
            .lines()
            .map(|line| line.map_err(|e| Error::ReadError(e)).unwrap())
            .collect::<Vec<String>>();

        Ok(words)
    }
}

// ===============================================

#[cfg(test)]
mod test {

    use super::*;

    #[ignore]
    #[test]
    fn basic() {
        let _wv = WordPool::new();
    }
}
