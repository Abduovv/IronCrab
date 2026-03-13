use crate::models::{Difficulty, Equipment, Exercise, MuscleGroup};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NinjasExercise {
    pub name:          String,
    #[serde(rename = "type")]
    pub exercise_type: String,
    pub muscle:        String,
    pub difficulty:    String,
    pub instructions:  String,
    pub equipments:    Vec<String>,
    pub safety_info:   String,
}

const MUSCLES: &[&str] = &[
    "biceps", "triceps", "chest", "lats",
    "middle_back", "lower_back", "shoulders",
    "quadriceps", "hamstrings", "calves",
    "glutes", "abdominals", "forearms",
    "traps", "abductors", "adductors",
];

pub fn api_key() -> String {
    std::env::var("NINJAS_API_KEY")
        .expect("NINJAS_API_KEY environment variable not set")
}

pub async fn fetch_all_exercises() -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();
    let mut all_exercises: Vec<Exercise> = Vec::new();
    let api_key = api_key();

    for muscle in MUSCLES {
        println!("Fetching: {}", muscle);

        let response: Vec<NinjasExercise> = client
            .get("https://api.api-ninjas.com/v1/exercises")
            .header("X-Api-Key", &api_key)
            .query(&[("muscle", muscle), ("offset", &"0")])
            .send()
            .await?
            .json()
            .await?;

        for raw in response {
            all_exercises.push(Exercise::from(raw));
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }

    Ok(all_exercises)
}

pub async fn fetch_exercises_by_muscle(muscle: &str) -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();
    let api_key = api_key();

    let response: Vec<NinjasExercise> = client.get("https://api.api-ninjas.com/v1/exercises")
        .header("X-Api-Key", &api_key)
        .query(&[("muscle", muscle), ("offset", &"0")])
        .send()
        .await?
        .json()
        .await?;

    Ok(response.into_iter().map(Exercise::from).collect())
}

pub async fn fetch_exercises_by_equipment(equipment: &str) -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();
    let api_key = api_key();

    let response: Vec<NinjasExercise> = client
        .get("https://api.api-ninjas.com/v1/exercises")
        .header("X-Api-Key", &api_key)
        .query(&[("equipment", equipment), ("offset", &"0")])  
        .send()
        .await?
        .json()
        .await?;

    Ok(response.into_iter().map(Exercise::from).collect())
}

pub async fn fetch_exercises_by_difficulty(difficulty: &str) -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();
    let api_key = api_key();

    let response: Vec<NinjasExercise> = client
        .get("https://api.api-ninjas.com/v1/exercises")
        .header("X-Api-Key", &api_key)
        .query(&[("difficulty", difficulty), ("offset", &"0")])  
        .send()
        .await?
        .json()
        .await?;

    Ok(response.into_iter().map(Exercise::from).collect())
}

impl From<NinjasExercise> for Exercise {
    fn from(n: NinjasExercise) -> Self {
        Exercise {
            name:         n.name,
            primary:      MuscleGroup::from_str(&n.muscle),
            equipment:    n.equipments
                           .first()
                           .map(|e| Equipment::from_str(e))
                           .unwrap_or(Equipment::Bodyweight),
            difficulty:   Difficulty::from_str(&n.difficulty),
            instructions: n.instructions,
            safety_info:  n.safety_info,
        }
    }
}