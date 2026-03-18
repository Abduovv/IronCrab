// src/main.rs
mod models;
mod scraper;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Fetching all exercises...\n");
    let exercises: Vec<models::Exercise> = scraper::api_source::fetch_all_exercises().await?;

    println!("\n--- Results ---");
    for ex in &exercises {
        println!(
            "{} | {:?} | {:?} | {:?}",
            ex.name, ex.primary, ex.equipment, ex.difficulty
        );
    }

    println!("\nTotal exercises fetched: {}", exercises.len());
    Ok(())
}
