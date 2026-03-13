use crate::models::{Difficulty, Equipment, MuscleGroup};

#[derive(Debug, Clone)]
pub struct Exercise {
    pub name:         String,
    pub primary:      MuscleGroup,
    pub equipment:    Equipment,
    pub difficulty:   Difficulty,
    pub instructions: String,
    pub safety_info:  String,
}