use std::env;
use battle_sim::simulation::run_batch_simulation;
use battle_sim::analysis::{load_results_from_file, run_analysis};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command> [options]", args[0]);
        println!("Commands:");
        println!("  simulate <count> <batch_id> [group_profile] [solo_profile] [group_count] [solo_count]");
        println!("  analyze <results_file>");
        return;
    }

    match args[1].as_str() {
        "simulate" => {
            if args.len() < 4 {
                println!("Usage: {} simulate <count> <batch_id> [group_profile] [solo_profile] [group_count] [solo_count]", args[0]);
                return;
            }
            let count: usize = args[2].parse().expect("Invalid count");
            let batch_id: usize = args[3].parse().expect("Invalid batch_id");
            let group_profile = args.get(4).map_or("man", |s| s.as_str());
            let solo_profile = args.get(5).map_or("gorilla", |s| s.as_str());
            let group_count: usize = args.get(6).map_or(100, |s| s.parse().expect("Invalid group_count"));
            let solo_count: usize = args.get(7).map_or(1, |s| s.parse().expect("Invalid solo_count"));

            run_batch_simulation(count, batch_id, group_profile, solo_profile, group_count, solo_count);
        }
        "analyze" => {
            if args.len() < 3 {
                println!("Usage: {} analyze <results_file>", args[0]);
                return;
            }
            let results_file = &args[2];
            match load_results_from_file(results_file) {
                Ok(results) => {
                    let batch_id = results_file
                        .split('_')
                        .nth(2)
                        .and_then(|s| s.split('.').next())
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                    run_analysis(&results, batch_id);
                }
                Err(e) => println!("Failed to load results: {}", e),
            }
        }
        _ => println!("Unknown command: {}", args[1]),
    }
}
