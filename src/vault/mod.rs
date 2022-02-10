// ========================
// ===============================================
// ===============================================================================================

// use std::io;
use std::io::BufRead;
// use std::io::Read;
// use std::path;

// use std::collections::HashSet;

// ===============================================

#[derive(Debug)]
pub enum Error {
    OpenError(std::io::Error),
    ReadError(std::io::Error),
}

pub struct WordVault {
    path: String,
}

impl WordVault {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap();
        Self {
            path: format!("{}/.wordle/vault", home),
        }
    }

    pub fn read_vault(&self) -> Result<Vec<String>, Error> {
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
        let _wv = WordVault::new();
    }
}
