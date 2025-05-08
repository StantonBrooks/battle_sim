use crate::models::{BattleResult, Team};
use serde_json::from_reader;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::collections::HashMap;

pub fn load_results_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<BattleResult>> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let results = from_reader(reader)?;
    Ok(results)
}

pub fn run_analysis(results: &[BattleResult], batch_id: usize) {
    let total = results.len() as f64;
    let group_wins = results.iter().filter(|r| r.winner == Team::Group).count() as f64;
    let solo_wins = results.iter().filter(|r| r.winner == Team::Solo).count() as f64;
    let avg_casualties = results.iter().map(|r| r.group_casualties).sum::<u32>() as f64 / total;
    let avg_rounds = results.iter().map(|r| r.rounds).sum::<u32>() as f64 / total;

    let mut climate_counts = HashMap::new();
    let mut weather_counts = HashMap::new();
    let mut day_count = 0;

    for r in results {
        *climate_counts.entry(&r.context.climate).or_insert(0) += 1;
        *weather_counts.entry(&r.context.weather).or_insert(0) += 1;
        if r.context.is_day {
            day_count += 1;
        }
    }

    let mut output = String::new();
    output.push_str("== Analysis Summary ==\n");
    output.push_str(&format!("Total Battles: {}\n\n", results.len()));
    output.push_str("Wins:\n");
    output.push_str(&format!("- Group: {:.1}%\n", (group_wins / total) * 100.0));
    output.push_str(&format!("- Solo: {:.1}%\n\n", (solo_wins / total) * 100.0));

    output.push_str(&format!("Average Group Casualties: {:.1}\n", avg_casualties));
    output.push_str(&format!("Average Rounds: {:.1}\n\n", avg_rounds));

    output.push_str("Climate Breakdown:\n");
    for (climate, count) in &climate_counts {
        output.push_str(&format!("- {}: {:.1}%\n", climate, (*count as f64 / total) * 100.0));
    }

    output.push_str("\nWeather Breakdown:\n");
    for (weather, count) in &weather_counts {
        output.push_str(&format!("- {}: {:.1}%\n", weather, (*count as f64 / total) * 100.0));
    }

    output.push_str("\nDay/Night:\n");
    output.push_str(&format!("- Day: {:.1}%\n", (day_count as f64 / total) * 100.0));
    output.push_str(&format!("- Night: {:.1}%\n", 100.0 - (day_count as f64 / total) * 100.0));

    let filename = format!("analysis_batch_{}.txt", batch_id);
    let mut file = File::create(&filename).expect("Failed to create analysis file");
    file.write_all(output.as_bytes()).expect("Failed to write analysis file");

    println!("Analysis written to {}", filename);
} 