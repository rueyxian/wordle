// ========================
// ===============================================
// ===============================================================================================

pub struct Permutations<'a, T> {
    elements: &'a [T],
    size: usize,
    indexes: Option<Vec<usize>>,
}

impl<'a, T> Permutations<'a, T>
where
    T: Copy,
{
    pub fn new(elements: &'a [T], size: usize) -> Self {
        Self {
            elements,
            size,
            indexes: None,
        }
    }

    fn permutation(&self) -> Option<Vec<T>> {
        self.indexes.as_ref().map(|indexes| {
            indexes
                .iter()
                .map(|&idx| self.elements[idx])
                .collect::<Vec<T>>()
        })
    }
}

impl<'a, T> Iterator for Permutations<'a, T>
where
    T: Copy,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.indexes {
            None => {
                self.indexes = Some(std::iter::repeat(0).take(self.size).collect::<Vec<usize>>());
                self.permutation()
            }

            Some(ref mut indexes) => {
                match indexes.iter().position(|&i| i + 1 < self.elements.len()) {
                    Some(position) => {
                        for index in indexes.iter_mut().take(position) {
                            *index = 0;
                        }
                        indexes[position] += 1;
                        self.permutation()
                    }
                    None => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Permutations;

    #[ignore]
    #[test]
    fn basic() {
        // let s = ["Adam", "Beth", "Cory"];
        let s = ["A", "B", "C"];
        let perms = Permutations::new(&s, 3);
        for p in perms {
            println!("{:?}", p);
        }
    }
}

// ===============================================================================================
