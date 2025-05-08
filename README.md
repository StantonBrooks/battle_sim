# 🦍 100 vs 1 Battle Simulation

Simulates 100 average adult human males engaging in hand-to-hand combat against a single silverback gorilla. Built in Rust using a modular system with deterministic behavior and parallelized batch execution.

## 🔧 Features
- Turn-based combat engine
- Grid-based spatial positioning
- Movement and targeting logic
- Stat-driven attacks with hit/miss and critical hit calculations
- Fatigue and endurance system
- Environmental effects (climate, weather, day/night)
- Coordinated group tactics
- Causal metrics tracking
- Parallel execution of thousands of battles
- JSON output of simulation results

## 📦 Project Structure
```
src/
├── agents/         # Agent behavior and combat stats
├── arena/          # Grid logic and collision checks
├── causal/         # Combat metrics and analysis
├── character_profiles/  # Combatant definitions
├── environment/    # Battle context and conditions
├── models/         # Shared structs and enums
├── output/         # Logging and analysis
├── simulation/     # Combat loop and batch executor
└── main.rs         # Entry point
```

## 🚀 Running the Simulation

### Step 1: Install Rust
If you don't have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Step 2: Build and Run
```bash
# Build the project
cargo build --release

# Run a batch of simulations
cargo run --release -- 1000 0 "Man" "Gorilla" 100 1
```

This runs:
- 1000 simulations
- Batch ID 0
- 100 humans vs 1 gorilla
- Results saved to `results_batch_0.json`

To run different scenarios, modify the parameters:
```bash
cargo run --release -- <simulations> <batch_id> <group_profile> <solo_profile> <group_count> <solo_count>
```

For example, to run 1,000,000 simulations of 50 humans vs 1 gorilla:
```bash
cargo run --release -- 1000000 1 "Man" "Gorilla" 50 1
```

## 📄 Output Format

Each result includes:
- `battle_id`: Simulation number
- `winner`: `Group` or `Solo`
- `rounds`: Number of turns taken
- `group_casualties`: Human deaths
- `solo_survived`: Whether the gorilla lived
- `context`: Battle environment (location, climate, weather, time)
- `causal`: Detailed combat metrics including:
  - Total critical hits
  - Group average damage
  - Maximum group damage
  - Solo end HP
  - Rounds engaged
  - Solo final blow

Output is saved as a formatted JSON array.

## 📊 Analyzing Results

After running a simulation batch, you can analyze the results using the analysis script:

```bash
# Analyze the results from the previous simulation
cargo run --release -- analyze results_batch_0.json

# For a specific batch
cargo run --release -- analyze results_batch_1.json
```

The analysis will output:
- Win rates for both groups
- Average casualties and rounds
- Climate and weather breakdown
- Day/night distribution
- Detailed combat metrics

Example output:
```
== Analysis Summary ==
Total Battles: 1000

Wins:
- Group: 0.1%
- Solo: 99.9%

Average Group Casualties: 3.6
Average Rounds: 39.3

Climate Breakdown:
- Temperate: 40.0%
- Continental: 30.0%
- Tropical: 10.0%
- Polar: 20.0%

Weather Breakdown:
- Clear: 20.0%
- Snow: 30.0%
- Cloudy: 20.0%
- Windy: 10.0%
- Rain: 10.0%
- Storm: 10.0%

Day/Night:
- Day: 70.0%
- Night: 30.0%
```

