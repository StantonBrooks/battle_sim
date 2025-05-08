use crate::models::{Agent, Team};
use crate::character_profiles::AgentProfile;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;

impl Agent {
    pub fn new_from_profile(id: usize, team: Team, x: i32, y: i32, profile: &AgentProfile) -> Self {
        Agent {
            id,
            team,
            x,
            y,
            hp: profile.hp,
            str_: profile.str_,
            spd: profile.spd,
            def: profile.def,
            base_damage: profile.base_damage,
            crit_chance: profile.crit_chance,
            alive: true,
            damage_dealt: 0,
        }
    }

    pub fn select_target(&self, agents: &[Agent]) -> Option<usize> {
        agents.iter()
            .enumerate()
            .filter(|(_, a)| a.alive && a.team != self.team)
            .min_by_key(|(_, a)| self.distance_to(a))
            .map(|(i, _)| i)
    }

    pub fn attack(&mut self, target: &mut Agent) -> (bool, i32) {
        let mut rng = rand::thread_rng();
        let hit_chance = 75 + (self.spd - target.spd) * 10;
        let roll = rng.gen_range(1..=100);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("combat.log")
            .unwrap();

        writeln!(file, "Attack: {} vs {} - Hit chance: {}, Roll: {}", self.id, target.id, hit_chance, roll).unwrap();

        if roll <= hit_chance {
            let mut damage = (self.base_damage + self.str_ * 2 - target.def / 2).max(5);
            let crit_roll = rng.gen_range(1..=100);
            let crit_threshold = (self.crit_chance + 10).clamp(1, 100);

            writeln!(file, "Hit! Base damage: {}, Crit roll: {}, Crit threshold: {}", damage, crit_roll, crit_threshold).unwrap();

            let crit = crit_roll <= crit_threshold;
            if crit {
                damage *= 2;
                writeln!(file, "Critical hit! Damage doubled to {}", damage).unwrap();
            }

            let actual_damage = damage.min(target.hp);
            writeln!(file, "Target HP before: {}, Damage dealt: {}", target.hp, actual_damage).unwrap();
            target.take_damage(actual_damage);
            writeln!(file, "Target HP after: {}", target.hp).unwrap();
            
            // Only add to damage_dealt if we actually hit and dealt damage
            self.damage_dealt += actual_damage as u32;
            (crit, actual_damage)
        } else {
            writeln!(file, "Miss!").unwrap();
            (false, 0)
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
        if self.hp <= 0 {
            self.alive = false;
        }
    }

    pub fn distance_to(&self, other: &Agent) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn move_towards_coords(&mut self, tx: i32, ty: i32, occupied: &[(i32, i32)]) {
        let dx = (tx - self.x).signum();
        let dy = (ty - self.y).signum();

        let options = vec![(self.x + dx, self.y), (self.x, self.y + dy)];
        for (nx, ny) in options {
            if !occupied.contains(&(nx, ny)) {
                self.x = nx;
                self.y = ny;
                break;
            }
        }
    }
} 