use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AgentProfile {
    pub hp: i32,
    pub str_: i32,
    pub spd: i32,
    pub def: i32,
    pub base_damage: i32,
    pub crit_chance: i32,
    pub endurance: i32,
    pub dexterity: i32,
    pub int_abstract: i32,
    pub int_environmental: i32,
    pub pain_tolerance: i32,
    pub behavior_flags: Vec<String>,
}

pub fn load_profiles(path: &str) -> HashMap<String, AgentProfile> {
    let file = File::open(path).expect("Could not open profile file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to deserialize profiles")
}

pub fn get_profile<'a>(profiles: &'a HashMap<String, AgentProfile>, name: &str) -> &'a AgentProfile {
    profiles.get(name).expect("Profile not found")
} 