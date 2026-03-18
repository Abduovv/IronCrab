use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MuscleGroup {
    Biceps,
    Triceps,
    Chest,
    Lats,
    MiddleBack,
    LowerBack,
    Shoulders,
    Quadriceps,
    Hamstrings,
    Calves,
    Glutes,
    Abdominals,
    Forearms,
    Traps,
    Abductors,
    Adductors,
    Unknown,
}

impl MuscleGroup {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "biceps" => Self::Biceps,
            "triceps" => Self::Triceps,
            "chest" => Self::Chest,
            "lats" => Self::Lats,
            "middle_back" | "middle back" => Self::MiddleBack,
            "lower_back" | "lower back" => Self::LowerBack,
            "shoulders" => Self::Shoulders,
            "quadriceps" => Self::Quadriceps,
            "hamstrings" => Self::Hamstrings,
            "calves" => Self::Calves,
            "glutes" => Self::Glutes,
            "abdominals" => Self::Abdominals,
            "forearms" => Self::Forearms,
            "traps" => Self::Traps,
            "abductors" => Self::Abductors,
            "adductors" => Self::Adductors,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for MuscleGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Biceps => "biceps",
            Self::Triceps => "triceps",
            Self::Chest => "chest",
            Self::Lats => "lats",
            Self::MiddleBack => "middle_back",
            Self::LowerBack => "lower_back",
            Self::Shoulders => "shoulders",
            Self::Quadriceps => "quadriceps",
            Self::Hamstrings => "hamstrings",
            Self::Calves => "calves",
            Self::Glutes => "glutes",
            Self::Abdominals => "abdominals",
            Self::Forearms => "forearms",
            Self::Traps => "traps",
            Self::Abductors => "abductors",
            Self::Adductors => "adductors",
            Self::Unknown => "unknown",
        };
        write!(f, "{}", s)
    }
}
