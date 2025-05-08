use crate::arena::*;
use crate::models::{Agent, BattleResult, Team};
use crate::output::*;
use crate::character_profiles::{load_profiles, get_profile, AgentProfile};
use crate::environment::BattleContext;
use crate::causal::CausalMetrics;
use rand::Rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;

pub fn run_batch_simulation(
    n: usize,
    batch_id: usize,
    group_profile_id: &str,
    solo_profile_id: &str,
    group_count: usize,
    solo_count: usize,
) {
    let profiles = load_profiles("combatants.json");
    let group_profile = get_profile(&profiles, group_profile_id);
    let solo_profile = get_profile(&profiles, solo_profile_id);

    let results: Vec<BattleResult> = (0..n)
        .into_par_iter()
        .map(|id| run_single_simulation(id, group_profile, solo_profile, group_count, solo_count))
        .collect();

    export_results(&results, batch_id);
}

pub fn run_single_simulation(
    battle_id: usize,
    group_profile: &AgentProfile,
    solo_profile: &AgentProfile,
    group_count: usize,
    solo_count: usize,
) -> BattleResult {
    let mut agents = init_agents(group_profile, solo_profile, group_count, solo_count);
    let mut round_count = 0;
    let mut causal = CausalMetrics::new();
    let mut last_attacker_id: Option<usize> = None;
    let mut consecutive_no_damage = 0;

    while simulation_active(&agents) && round_count < 1000 {
        round_count += 1;
        let round_damage = execute_round(&mut agents, &mut causal, &mut last_attacker_id);
        
        if round_damage > 0 {
            causal.rounds_engaged += 1;
            consecutive_no_damage = 0;
        } else {
            consecutive_no_damage += 1;
            // If no damage has been dealt for 10 consecutive rounds, end the battle
            if consecutive_no_damage >= 10 {
                break;
            }
        }
    }

    summarize_battle(battle_id, &agents, round_count, causal)
}

fn init_agents(
    group_profile: &AgentProfile,
    solo_profile: &AgentProfile,
    group_count: usize,
    solo_count: usize,
) -> Vec<Agent> {
    let mut agents = Vec::new();
    let mut rng = rand::thread_rng();

    let mut positions: Vec<(i32, i32)> = (0..ARENA_WIDTH)
        .flat_map(|x| (0..ARENA_HEIGHT).map(move |y| (x, y)))
        .collect();
    positions.shuffle(&mut rng);

    for i in 0..group_count {
        let (x, y) = positions.pop().unwrap();
        agents.push(Agent::new_from_profile(i, Team::Group, x, y, group_profile));
    }
    for j in 0..solo_count {
        let id = group_count + j;
        let (x, y) = positions.pop().unwrap();
        agents.push(Agent::new_from_profile(id, Team::Solo, x, y, solo_profile));
    }

    agents
}

fn simulation_active(agents: &Vec<Agent>) -> bool {
    let group_alive = agents.iter().any(|a| a.team == Team::Group && a.alive);
    let solo_alive = agents.iter().any(|a| a.team == Team::Solo && a.alive);
    group_alive && solo_alive
}

fn execute_round(agents: &mut Vec<Agent>, causal: &mut CausalMetrics, last_attacker_id: &mut Option<usize>) -> i32 {
    let mut round_damage = 0;
    let mut rng = rand::thread_rng();
    
    // Sort agents by team to allow coordinated attacks
    let mut order: Vec<usize> = (0..agents.len()).collect();
    order.sort_by_key(|&i| if agents[i].team == Team::Group { 0 } else { 1 });
    order.shuffle(&mut rng);

    let occupied: Vec<(i32, i32)> = agents
        .iter()
        .map(|a| (a.x, a.y))
        .collect();

    // Track which agents have already been targeted this round
    let mut targeted_agents = std::collections::HashSet::new();

    for i in order {
        if !agents[i].alive {
            continue;
        }

        // Calculate fatigue based on rounds engaged
        let fatigue_penalty = if causal.rounds_engaged > 0 {
            // More fatigue for solo agent (gorilla)
            let base_fatigue = if agents[i].team == Team::Solo { 3 } else { 1 };
            // Exponential fatigue growth
            let raw_fatigue = (causal.rounds_engaged as f32).powf(1.2) * base_fatigue as f32;
            // Cap fatigue at 50% of base stats
            let max_fatigue = if agents[i].team == Team::Solo {
                (agents[i].spd / 2) as f32
            } else {
                (agents[i].spd / 3) as f32
            };
            raw_fatigue.min(max_fatigue) as i32
        } else {
            0
        };

        // Apply fatigue effects
        let effective_speed = (agents[i].spd - fatigue_penalty).max(1);
        let effective_strength = (agents[i].str_ - (fatigue_penalty * 2)).max(1);
        let effective_defense = (agents[i].def - (fatigue_penalty * 3)).max(1);

        // If this is a group agent, try to coordinate with nearby allies
        if agents[i].team == Team::Group {
            let nearby_allies: Vec<&Agent> = agents.iter()
                .filter(|a| a.team == Team::Group && a.alive && a.id != agents[i].id)
                .filter(|a| agents[i].distance_to(a) <= 2)
                .collect();

            // If we have nearby allies, increase our hit chance and damage
            let ally_bonus = nearby_allies.len() as i32;
            let target_id = agents[i].select_target(&agents);
            if target_id.is_none() {
                continue;
            }

            let target_id = target_id.unwrap();
            if targeted_agents.contains(&target_id) {
                continue; // Skip if target was already attacked this round
            }

            let target = agents.iter().find(|a| a.id == target_id).unwrap();
            let (tx, ty) = (target.x, target.y);

            if agents[i].distance_to(target) > 1 {
                agents[i].move_towards_coords(tx, ty, &occupied);
            } else {
                // More nuanced hit chance calculation with fatigue
                let base_hit = 60 + (effective_speed - target.spd) * 5;
                let ally_hit_bonus = ally_bonus * 3;
                let hit_chance = (base_hit + ally_hit_bonus + rng.gen_range(-10..=10)).clamp(20, 90);
                let roll: i32 = rng.gen_range(1..=100);
                
                if roll <= hit_chance {
                    // More nuanced damage calculation with fatigue
                    let base_damage = agents[i].base_damage;
                    let strength_bonus = effective_strength;
                    let defense_reduction = effective_defense / (2 + ally_bonus);
                    let damage = (base_damage + strength_bonus - defense_reduction).max(2);
                    
                    // Critical hit calculation with fatigue
                    let crit_roll: i32 = rng.gen_range(1..=100);
                    let base_crit = agents[i].crit_chance;
                    let ally_crit_bonus = ally_bonus * 2;
                    let crit_threshold = (base_crit + ally_crit_bonus + rng.gen_range(-5..=5)).clamp(1, 95);
                    
                    let total_damage = if crit_roll <= crit_threshold {
                        causal.total_critical_hits += 1;
                        damage * 2
                    } else {
                        damage
                    };

                    let (attacker, target) = if i < target_id {
                        let (left, right) = agents.split_at_mut(target_id);
                        (&mut left[i], &mut right[0])
                    } else {
                        let (left, right) = agents.split_at_mut(i);
                        (&mut right[0], &mut left[target_id])
                    };

                    target.take_damage(total_damage);
                    attacker.damage_dealt += total_damage as u32;
                    round_damage += total_damage;
                    targeted_agents.insert(target_id);

                    if !target.alive {
                        *last_attacker_id = Some(attacker.id);
                    }
                }
            }
        } else {
            // Solo agent (gorilla) behavior with fatigue
            let target_id = agents[i].select_target(&agents);
            if target_id.is_none() {
                continue;
            }

            let target_id = target_id.unwrap();
            let target = agents.iter().find(|a| a.id == target_id).unwrap();
            let (tx, ty) = (target.x, target.y);

            if agents[i].distance_to(target) > 1 {
                agents[i].move_towards_coords(tx, ty, &occupied);
            } else {
                // Gorilla-specific hit chance with fatigue
                let base_hit = 65 + (effective_speed - target.spd) * 8;
                let hit_chance = (base_hit + rng.gen_range(-15..=15)).clamp(25, 95);
                let roll: i32 = rng.gen_range(1..=100);
                
                if roll <= hit_chance {
                    // Gorilla-specific damage with fatigue
                    let base_damage = agents[i].base_damage;
                    let strength_bonus = effective_strength * 2;
                    let defense_reduction = effective_defense / 2;
                    let damage = (base_damage + strength_bonus - defense_reduction).max(5);
                    
                    // Gorilla critical hits with fatigue
                    let crit_roll: i32 = rng.gen_range(1..=100);
                    let crit_threshold = (agents[i].crit_chance + rng.gen_range(-10..=10)).clamp(1, 95);
                    
                    let total_damage = if crit_roll <= crit_threshold {
                        causal.total_critical_hits += 1;
                        damage * 2
                    } else {
                        damage
                    };

                    let (attacker, target) = if i < target_id {
                        let (left, right) = agents.split_at_mut(target_id);
                        (&mut left[i], &mut right[0])
                    } else {
                        let (left, right) = agents.split_at_mut(i);
                        (&mut right[0], &mut left[target_id])
                    };

                    target.take_damage(total_damage);
                    attacker.damage_dealt += total_damage as u32;
                    round_damage += total_damage;
                    targeted_agents.insert(target_id);

                    if !target.alive {
                        *last_attacker_id = Some(attacker.id);
                    }
                }
            }
        }
    }

    round_damage
}

fn summarize_battle(battle_id: usize, agents: &Vec<Agent>, rounds: u32, causal: CausalMetrics) -> BattleResult {
    let solo_alive = agents.iter().any(|a| a.team == Team::Solo && a.alive);
    let group_casualties = agents
        .iter()
        .filter(|a| a.team == Team::Group && !a.alive)
        .count() as u32;

    let context = BattleContext::random_from_file("realistic_cities_with_climate.csv");

    let mut result = BattleResult {
        battle_id,
        winner: if solo_alive { Team::Solo } else { Team::Group },
        rounds,
        group_casualties,
        solo_survived: solo_alive,
        context,
        causal,
    };

    // Calculate group average damage and max group damage
    let group_damage: Vec<u32> = agents
        .iter()
        .filter(|a| a.team == Team::Group)
        .map(|a| a.damage_dealt)
        .collect();

    if !group_damage.is_empty() {
        result.causal.group_avg_damage = group_damage.iter().sum::<u32>() as f32 / group_damage.len() as f32;
        result.causal.max_group_damage = *group_damage.iter().max().unwrap() as f32;
    }

    // Set solo end HP
    if let Some(solo) = agents.iter().find(|a| a.team == Team::Solo) {
        result.causal.solo_end_hp = if solo.alive { solo.hp.max(0) as u32 } else { 0 };
    }

    result
} 