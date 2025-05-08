use crate::models::Agent;
use std::collections::HashSet;
use rand::Rng;

pub const ARENA_WIDTH: i32 = 100;
pub const ARENA_HEIGHT: i32 = 100;

/// Returns true if the (x, y) coordinate is within the 50x50 arena bounds
pub fn is_within_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < ARENA_WIDTH && y >= 0 && y < ARENA_HEIGHT
}

/// Returns a HashSet of positions currently occupied by alive agents
pub fn get_occupied_positions(agents: &Vec<Agent>) -> HashSet<(i32, i32)> {
    agents
        .iter()
        .filter(|a| a.alive)
        .map(|a| (a.x, a.y))
        .collect()
}

pub struct Arena {
    width: i32,
    height: i32,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            width: 100,
            height: 100,
        }
    }

    pub fn random_position() -> (i32, i32) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..100), rng.gen_range(0..100))
    }

    pub fn update_positions(&mut self, agents: &mut [crate::models::Agent]) {
        // For now, this is a placeholder. We can implement more complex movement logic later.
        for agent in agents {
            if agent.alive {
                // Simple random movement
                let mut rng = rand::thread_rng();
                let dx = rng.gen_range(-1..=1);
                let dy = rng.gen_range(-1..=1);
                agent.x = (agent.x + dx).clamp(0, self.width - 1);
                agent.y = (agent.y + dy).clamp(0, self.height - 1);
            }
        }
    }
} 