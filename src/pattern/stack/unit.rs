#[derive(Debug, Clone)]
pub enum Unit {
    Correct(char),
    Wrong(char),
    NotAny(char),
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Correct(c) => write!(f, "!{}", c),
            Unit::Wrong(c) => write!(f, "?{}", c),
            Unit::NotAny(c) => write!(f, "#{}", c),
        }
    }
}
