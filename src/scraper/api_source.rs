use crate::models::{Equipment, Exercise, MuscleGroup};
use serde::Deserialize;

// ── Raw API response structs ──────────────────────────────────────────────────

#[derive(Deserialize, Debug)]
pub struct ExerciseDbResponse {
    pub success: bool,
    pub metadata: Metadata,
    pub data: Vec<ExerciseDbExercise>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub total_exercises: u32,
    pub total_pages: u32,
    pub current_page: u32,
    pub previous_page: Option<String>,
    pub next_page: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseDbExercise {
    pub exercise_id: String,
    pub name: String,
    pub gif_url: String,
    pub target_muscles: Vec<String>,
    pub body_parts: Vec<String>,
    pub equipments: Vec<String>,
    pub secondary_muscles: Vec<String>,
    pub instructions: Vec<String>,
}

// ── Conversion ────────────────────────────────────────────────────────────────

impl From<ExerciseDbExercise> for Exercise {
    fn from(e: ExerciseDbExercise) -> Self {
        Exercise {
            name: e.name,
            primary: e
                .target_muscles
                .first()
                .map(|m| MuscleGroup::from_str(m))
                .unwrap_or(MuscleGroup::Unknown),
            equipment: e
                .equipments
                .first()
                .map(|eq| Equipment::from_str(eq))
                .unwrap_or(Equipment::Bodyweight),
            instructions: e.instructions.join("\n"),
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

const BASE_URL: &str = "https://exercisedb.dev/api/v1/exercises";
const PAGE_LIMIT: u32 = 10;

// ── Fetch functions ───────────────────────────────────────────────────────────

pub async fn fetch_all_exercises() -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();
    let mut all_exercises: Vec<Exercise> = Vec::new();
    let mut offset: u32 = 0;

    loop {
        let response: ExerciseDbResponse = client
            .get(BASE_URL)
            .query(&[
                ("limit", PAGE_LIMIT.to_string()),
                ("offset", offset.to_string()),
            ])
            .send()
            .await?
            .json()
            .await?;

        let total_pages = response.metadata.total_pages;
        let current_page = response.metadata.current_page;

        for raw in response.data {
            all_exercises.push(Exercise::from(raw));
        }

        if current_page >= total_pages {
            break;
        }

        offset += PAGE_LIMIT;
    }

    Ok(all_exercises)
}

pub async fn fetch_exercises_by_muscle(muscle: &str) -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();

    let response: ExerciseDbResponse = client
        .get(format!(
            "https://exercisedb.dev/api/v1/muscles/{}/exercises",
            muscle
        ))
        .send()
        .await?
        .json()
        .await?;

    Ok(response.data.into_iter().map(Exercise::from).collect())
}

pub async fn fetch_exercises_by_bodypart(bodypart: &str) -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();

    let response: ExerciseDbResponse = client
        .get(format!(
            "https://exercisedb.dev/api/v1/bodyparts/{}/exercises",
            bodypart
        ))
        .send()
        .await?
        .json()
        .await?;

    Ok(response.data.into_iter().map(Exercise::from).collect())
}

pub async fn fetch_exercises_by_equipment(equipment: &str) -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();

    let response: ExerciseDbResponse = client
        .get(format!(
            "https://exercisedb.dev/api/v1/equipments/{}/exercises",
            equipment
        ))
        .send()
        .await?
        .json()
        .await?;

    Ok(response.data.into_iter().map(Exercise::from).collect())
}

pub async fn fetch_all_muscles() -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();

    let request = client.get("https://exercisedb.dev/api/v1/muscles");

    let response = request.send().await?;

    Ok(response.data.into_iter().map(Exercise::from).collect())
}

pub async fn fetch_all_equipments() -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();

    let request = client.get("https://exercisedb.dev/api/v1/equipments");

    let response = request.send().await?;

    Ok(response.data.into_iter().map(Exercise::from).collect())
}

pub async fn fetch_all_bodyparts() -> anyhow::Result<Vec<Exercise>> {
    let client = reqwest::Client::new();

    let request = client.get("https://exercisedb.dev/api/v1/bodyparts");

    let response = request.send().await?;

    Ok(response.data.into_iter().map(Exercise::from).collect())
}
