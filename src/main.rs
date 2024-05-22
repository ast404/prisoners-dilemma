mod combinatorics;
mod game;
mod player;
mod population;
mod strategy;
mod tournament;

use population::simulate_populations;
use strategy::all_strategies;

fn main() {
    let strategies = all_strategies();
    let strategy_wins = simulate_populations(&strategies, /* max_player_instances= */ 5);
    for (strategy, wins_num) in &strategy_wins {
        println!("{}: {}", strategy, wins_num);
    }
}
