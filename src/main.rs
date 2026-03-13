// src/main.rs
mod models;
mod scraper;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("Fetching all exercises...\n");
    let exercises = scraper::api_ninjas::fetch_all_exercises().await?;

    println!("\n--- Results ---");
    for ex in &exercises {
        println!("{} | {:?} | {:?} | {:?}", 
            ex.name, ex.primary, ex.equipment, ex.difficulty);
    }

    println!("\nTotal exercises fetched: {}", exercises.len());
    Ok(())
}