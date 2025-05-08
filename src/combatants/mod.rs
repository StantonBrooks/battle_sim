use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatantStats {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatantDefinitions {
    #[serde(rename = "Man")]
    pub man: CombatantStats,
    #[serde(rename = "Gorilla")]
    pub gorilla: CombatantStats,
}

impl CombatantDefinitions {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let json_str = fs::read_to_string("src/combatants.json")?;
        let definitions: CombatantDefinitions = serde_json::from_str(&json_str)?;
        Ok(definitions)
    }

    pub fn get_stats(&self, combatant_type: &str) -> Option<&CombatantStats> {
        match combatant_type {
            "Man" => Some(&self.man),
            "Gorilla" => Some(&self.gorilla),
            _ => None,
        }
    }
} 