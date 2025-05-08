use crate::models::{BattleResult, Team};
use serde_json;
use std::fs::File;
use std::path::Path;

pub fn log_battle_result(result: &BattleResult) {
    println!("Battle Result:");
    println!("Winner: {:?}", result.winner);
    println!("Rounds: {}", result.rounds);
    println!("Group Casualties: {}", result.group_casualties);
    println!("Solo Survived: {}", result.solo_survived);
}

pub fn analyze_results(results: &[BattleResult]) {
    let total_simulations = results.len() as f32;
    let average_rounds = results.iter().map(|r| r.rounds as f32).sum::<f32>() / total_simulations;
    let average_casualties = results.iter().map(|r| r.group_casualties as f32).sum::<f32>() / total_simulations;
    let solo_survival_rate = results.iter().filter(|r| r.solo_survived).count() as f32 / total_simulations;

    println!("\nAnalysis of {} simulations:", total_simulations);
    println!("Average rounds per battle: {:.2}", average_rounds);
    println!("Average group casualties: {:.2}", average_casualties);
    println!("Solo survival rate: {:.2}%", solo_survival_rate * 100.0);
}

pub fn save_results_to_file(results: &[BattleResult], filename: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    std::fs::write(filename, json)
}

pub fn load_results(path: &str) -> Result<Vec<BattleResult>, Box<dyn std::error::Error>> {
    let path = Path::new(path);
    let mut results = Vec::new();

    if path.is_file() {
        // Load single file
        let file = File::open(path)?;
        let batch_results: Vec<BattleResult> = serde_json::from_reader(file)?;
        results.extend(batch_results);
    } else if path.is_dir() {
        // Load all JSON files from directory
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let file = File::open(&path)?;
                let mut batch_results: Vec<BattleResult> = serde_json::from_reader(file)?;
                results.append(&mut batch_results);
            }
        }
    } else {
        return Err(format!("Path '{}' does not exist", path.display()).into());
    }

    Ok(results)
}

pub fn export_results(results: &[BattleResult], batch_id: usize) {
    let path = format!("results_batch_{}.json", batch_id);
    let file = File::create(&path).expect("Failed to create results file");
    serde_json::to_writer_pretty(file, results).expect("Failed to write results");
}

pub fn print_results(results: &[BattleResult]) {
    let total_battles = results.len();
    let group_wins = results.iter().filter(|r| r.winner == Team::Group).count();
    let solo_wins = results.iter().filter(|r| r.winner == Team::Solo).count();
    let avg_rounds = results.iter().map(|r| r.rounds as f64).sum::<f64>() / total_battles as f64;
    let avg_casualties = results.iter().map(|r| r.group_casualties as f64).sum::<f64>() / total_battles as f64;
    let solo_survival_rate = results.iter().filter(|r| r.solo_survived).count() as f64 / total_battles as f64;

    println!("\nSimulation Results:");
    println!("Total Battles: {}", total_battles);
    println!("Group Wins: {} ({:.1}%)", group_wins, (group_wins as f64 / total_battles as f64) * 100.0);
    println!("Solo Wins: {} ({:.1}%)", solo_wins, (solo_wins as f64 / total_battles as f64) * 100.0);
    println!("Average Rounds: {:.1}", avg_rounds);
    println!("Average Group Casualties: {:.1}", avg_casualties);
    println!("Solo Survival Rate: {:.1}%", solo_survival_rate * 100.0);

    println!("\nBattle Context:");
    if let Some(result) = results.first() {
        println!("Location: {}, {}", result.context.location_name, result.context.country);
        println!("Lat/Lon: ({:.4}, {:.4})", result.context.latitude, result.context.longitude);
        println!("Climate: {}, Weather: {}, Day: {}", result.context.climate, result.context.weather, result.context.is_day);
    }
} 