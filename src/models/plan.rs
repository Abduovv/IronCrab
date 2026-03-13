use crate::models::WorkoutDay;

#[derive(Debug, Clone)]
pub struct Plan {
    pub days_per_week: u8,
    pub goal:          String,
    pub days:          Vec<WorkoutDay>,
}