use crate::models::{Equipment, MuscleGroup};

#[derive(Debug, Clone)]
pub struct Exercise {
    pub name: String,
    pub primary: MuscleGroup,
    pub equipment: Equipment,
    pub instructions: String,
}
