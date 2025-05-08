use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, Deserialize)]
pub struct CityClimateEntry {
    pub city: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub climate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleContext {
    pub location_name: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub climate: String,
    pub weather: String,
    pub is_day: bool,
}

impl BattleContext {
    pub fn random_from_file(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open climate CSV");
        let mut rdr = csv::Reader::from_reader(BufReader::new(file));
        let entries: Vec<CityClimateEntry> = rdr
            .deserialize()
            .map(|res| res.expect("Failed to parse row"))
            .collect();

        let mut rng = rand::thread_rng();
        let entry = entries.choose(&mut rng).expect("No entries in climate CSV");

        let weather_options = get_weather_options(&entry.climate);
        let weather = weather_options.choose(&mut rng).unwrap_or(&"Clear").to_string();
        let is_day = rng.gen_bool(0.5);

        BattleContext {
            location_name: entry.city.clone(),
            country: entry.country.clone(),
            latitude: entry.latitude,
            longitude: entry.longitude,
            climate: entry.climate.clone(),
            weather,
            is_day,
        }
    }
}

fn get_weather_options(climate: &str) -> Vec<&'static str> {
    match climate {
        "Tropical" => vec!["Humid", "Rain", "Clear", "Storm"],
        "Arid" => vec!["Clear", "Windy", "Dusty", "Hot"],
        "Temperate" => vec!["Clear", "Rain", "Cloudy", "Windy"],
        "Continental" => vec!["Snow", "Overcast", "Clear", "Rain"],
        "Polar" => vec!["Snow", "Blizzard", "Freezing Fog", "Clear"],
        _ => vec!["Clear"],
    }
} 