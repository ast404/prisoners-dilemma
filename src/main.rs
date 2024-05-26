mod combinatorics;
mod game;
mod player;
mod population;
mod strategy;
mod tournament;

use population::simulate_populations;
use std::env;
use strategy::all_strategies;
use tournament::Tournament;

// TODO: Figure out how to do flags in Rust in a better manner.
fn create_tournament(args: &[String]) -> Tournament {
    if args.len() > 1 && args[1] == "axelrod" {
        Tournament::axelrod_tournament()
    } else {
        Tournament::all_pairs()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let tournament = create_tournament(&args);
    let strategies = all_strategies();
    let strategy_wins =
        simulate_populations(&strategies, /* max_player_instances= */ 3, &tournament);
    for (strategy, wins_num) in &strategy_wins {
        println!("{}: {}", strategy, wins_num);
    }
}
