use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Equipment {
    Barbell,
    Dumbbell,
    Bodyweight,
    Cable,
    Machine,
    Kettlebell,
    ResistanceBand,
    Other(String),
}

impl Equipment {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "barbell" => Self::Barbell,
            "dumbbell" | "dumbbells" => Self::Dumbbell,
            "body only" | "body_only" | "bodyweight" => Self::Bodyweight,
            "cable" | "cables" => Self::Cable,
            "machine" => Self::Machine,
            "kettlebells" | "kettlebell" => Self::Kettlebell,
            "bands" | "resistance band" | "resistance_band" => Self::ResistanceBand,
            other => Self::Other(other.to_string()),
        }
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Barbell => "barbell".to_string(),
            Self::Dumbbell => "dumbbell".to_string(),
            Self::Bodyweight => "bodyweight".to_string(),
            Self::Cable => "cable".to_string(),
            Self::Machine => "machine".to_string(),
            Self::Kettlebell => "kettlebell".to_string(),
            Self::ResistanceBand => "resistance_band".to_string(),
            Self::Other(s) => s.clone(),
        };
        write!(f, "{}", s)
    }
}
