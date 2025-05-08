use serde::{Deserialize, Serialize};
use crate::environment::BattleContext;
use crate::causal::CausalMetrics;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Team {
    Group,  // Human group
    Solo,   // Gorilla
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: usize,
    pub team: Team,
    pub hp: i32,
    pub str_: i32,
    pub spd: i32,
    pub def: i32,
    pub base_damage: i32,
    pub crit_chance: i32,
    pub x: i32,
    pub y: i32,
    pub alive: bool,
    pub damage_dealt: u32,
}

impl Default for Agent {
    fn default() -> Self {
        Agent {
            id: 0,
            team: Team::Group,
            hp: 0,
            str_: 0,
            spd: 0,
            def: 0,
            base_damage: 0,
            crit_chance: 0,
            x: 0,
            y: 0,
            alive: true,
            damage_dealt: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleResult {
    pub battle_id: usize,
    pub winner: Team,
    pub rounds: u32,
    pub group_casualties: u32,
    pub solo_survived: bool,
    pub context: BattleContext,
    pub causal: CausalMetrics,
}
