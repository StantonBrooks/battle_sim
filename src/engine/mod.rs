use crate::arena::Arena;
use crate::models::{Agent, BattleResult, Team};
use crate::character_profiles::AgentProfile;
use crate::causal::CausalMetrics;
use std::fs::OpenOptions;
use std::io::Write;

pub fn run_battle(
    battle_id: usize,
    group_profile: &AgentProfile,
    solo_profile: &AgentProfile,
    group_count: usize,
    solo_count: usize,
) -> BattleResult {
    let mut agents = Vec::new();
    let mut causal = CausalMetrics::new();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("battle.log")
        .unwrap();

    writeln!(file, "Starting battle {} with {} group agents and {} solo agents", battle_id, group_count, solo_count).unwrap();

    for i in 0..group_count {
        let (x, y) = Arena::random_position();
        let mut agent = Agent::new_from_profile(i, Team::Group, x, y, group_profile);
        agent.damage_dealt = 0;
        agents.push(agent);
    }

    for i in 0..solo_count {
        let (x, y) = Arena::random_position();
        let mut agent = Agent::new_from_profile(10000 + i, Team::Solo, x, y, solo_profile);
        agent.damage_dealt = 0;
        agents.push(agent);
    }

    let mut arena = Arena::new();
    let mut round = 0;

    while round < 1000 {
        round += 1;
        writeln!(file, "\nRound {}", round).unwrap();
        arena.update_positions(&mut agents);

        let mut round_engaged = false;
        let mut round_damage = 0;
        let agent_ids: Vec<usize> = (0..agents.len()).collect();

        for i in agent_ids {
            if !agents[i].alive {
                continue;
            }

            let target_id_opt = agents[i].select_target(&agents);
            if let Some(target_id) = target_id_opt {
                if i == target_id || !agents[target_id].alive {
                    continue;
                }

                round_engaged = true;

                let (attacker, target) = if i < target_id {
                    let (left, right) = agents.split_at_mut(target_id);
                    (&mut left[i], &mut right[0])
                } else {
                    let (left, right) = agents.split_at_mut(i);
                    (&mut right[0], &mut left[target_id])
                };

                writeln!(file, "Agent {} (Team: {:?}) attacking Agent {} (Team: {:?})", 
                    attacker.id, attacker.team, target.id, target.team).unwrap();

                let (crit, damage_dealt) = attacker.attack(target);
                if crit {
                    causal.total_critical_hits += 1;
                    writeln!(file, "Critical hit registered! Total: {}", causal.total_critical_hits).unwrap();
                }
                if damage_dealt > 0 {
                    attacker.damage_dealt += damage_dealt as u32;
                    round_damage += damage_dealt;
                    writeln!(file, "Damage dealt: {}. Total damage for agent {}: {}", 
                        damage_dealt, attacker.id, attacker.damage_dealt).unwrap();
                }

                if !target.alive {
                    causal.solo_final_blow = attacker.team == Team::Solo;
                    writeln!(file, "Agent {} killed! Final blow by {:?}", target.id, attacker.team).unwrap();
                }
            }
        }

        if round_engaged {
            causal.rounds_engaged += 1;
            writeln!(file, "Round {} engaged with {} damage dealt", round, round_damage).unwrap();
        }

        if is_battle_over(&agents) {
            writeln!(file, "Battle over after {} rounds", round).unwrap();
            break;
        }
    }

    let group_alive = agents.iter().any(|a| a.alive && a.team == Team::Group);
    let solo_alive = agents.iter().any(|a| a.alive && a.team == Team::Solo);
    let group_casualties = group_count as u32 - agents.iter().filter(|a| a.alive && a.team == Team::Group).count() as u32;

    let winner = if solo_alive && !group_alive {
        Team::Solo
    } else {
        Team::Group
    };

    writeln!(file, "\nFinal stats:").unwrap();
    writeln!(file, "Winner: {:?}", winner).unwrap();
    writeln!(file, "Group casualties: {}", group_casualties).unwrap();
    writeln!(file, "Solo survived: {}", solo_alive).unwrap();

    let group_damage: Vec<f32> = agents.iter()
        .filter(|a| a.team == Team::Group)
        .map(|a| a.damage_dealt as f32)
        .collect();

    causal.group_avg_damage = if !group_damage.is_empty() {
        group_damage.iter().sum::<f32>() / group_damage.len() as f32
    } else {
        0.0
    };

    causal.max_group_damage = group_damage.into_iter().fold(0.0, |a, b| a.max(b));

    if let Some(solo) = agents.iter().find(|a| a.team == Team::Solo) {
        causal.solo_end_hp = solo.hp.max(0) as u32;
    }

    writeln!(file, "Causal metrics:").unwrap();
    writeln!(file, "Total critical hits: {}", causal.total_critical_hits).unwrap();
    writeln!(file, "Group avg damage: {}", causal.group_avg_damage).unwrap();
    writeln!(file, "Max group damage: {}", causal.max_group_damage).unwrap();
    writeln!(file, "Solo end HP: {}", causal.solo_end_hp).unwrap();
    writeln!(file, "Rounds engaged: {}", causal.rounds_engaged).unwrap();
    writeln!(file, "Solo final blow: {}", causal.solo_final_blow).unwrap();

    BattleResult {
        battle_id,
        winner,
        rounds: round,
        group_casualties,
        solo_survived: solo_alive,
        context: crate::environment::BattleContext::random_from_file("realistic_cities_with_climate.csv"),
        causal,
    }
}

fn is_battle_over(agents: &[Agent]) -> bool {
    let group_alive = agents.iter().any(|a| a.alive && a.team == Team::Group);
    let solo_alive = agents.iter().any(|a| a.alive && a.team == Team::Solo);
    !(group_alive && solo_alive)
}