use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalMetrics {
    pub total_critical_hits: u32,
    pub group_avg_damage: f32,
    pub max_group_damage: f32,
    pub solo_end_hp: u32,
    pub rounds_engaged: u32,
    pub solo_final_blow: bool,
}

impl CausalMetrics {
    pub fn new() -> Self {
        CausalMetrics {
            total_critical_hits: 0,
            group_avg_damage: 0.0,
            max_group_damage: 0.0,
            solo_end_hp: 0,
            rounds_engaged: 0,
            solo_final_blow: false,
        }
    }
} 