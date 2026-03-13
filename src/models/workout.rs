use crate::models::Exercise;

#[derive(Debug, Clone)]
pub struct WorkoutDay {
    pub name:      String,           // "Monday", "Push Day", etc.
    pub exercises: Vec<WorkoutSet>,
}

#[derive(Debug, Clone)]
pub struct WorkoutSet {
    pub exercise: Exercise,
    pub sets:     u8,
    pub reps:     u8,
}