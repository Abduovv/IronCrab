use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
    Unknown,
}

impl Difficulty {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "beginner"     => Self::Beginner,
            "intermediate" => Self::Intermediate,
            "expert"       => Self::Expert,
            _              => Self::Unknown,
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Beginner     => "beginner",
            Self::Intermediate => "intermediate",
            Self::Expert       => "expert",
            Self::Unknown      => "unknown",
        };
        write!(f, "{}", s)
    }
}