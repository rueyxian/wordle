#[derive(Debug, Clone)]
pub enum PatternUnit {
    Correct(char),
    Wrong(char),
    NotAny(char),
}

impl std::fmt::Display for PatternUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Unit::Correct(c) => write!(f, "!{}", c),
            // Unit::Wrong(c) => write!(f, "?{}", c),
            // Unit::NotAny(c) => write!(f, "#{}", c),
            PatternUnit::Correct(c) => write!(f, "🟩{}", c),
            PatternUnit::Wrong(c) => write!(f, "🟨{}", c),
            PatternUnit::NotAny(c) => write!(f, "⬛️{}", c),
        }
    }
}
