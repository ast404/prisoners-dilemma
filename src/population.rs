use crate::combinatorics::Combinations;
use crate::player::Player;
use crate::strategy::Strategy;
use crate::tournament::AllPairsTournament;
use std::collections::HashMap;

pub fn simulate_populations(
    strategies: &[Box<dyn Strategy>],
    max_player_instances: u8,
) -> HashMap<String, u32> {
    let mut strategy_wins = HashMap::new();
    let all_combinations =
        Combinations::new(strategies.len().try_into().unwrap(), max_player_instances);
    for player_counts in all_combinations {
        let mut players = create_players(strategies, &player_counts);
        let winning_strategy = get_winning_strategy(&mut players);
        strategy_wins
            .entry(winning_strategy)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    strategy_wins
}

fn get_winning_strategy(players: &mut [Player]) -> String {
    let tournament = AllPairsTournament::new();
    tournament.play_games(players);
    let best_player = players.iter().max_by_key(|p| p.score());
    best_player.expect("at least one player").strategy_name()
}

fn create_players<'a>(
    strategies: &'a [Box<dyn Strategy>],
    player_counts: &'a [u8],
) -> Vec<Player<'a>> {
    if strategies.len() != player_counts.len() {
        panic!("player counts don't match the size of strategies");
    }
    let mut players: Vec<Player> = Vec::new();
    for i in 0..strategies.len() {
        for j in 0..(player_counts[i] + 1) {
            players.push(Player::new(
                &format!("{}_{}", strategies[i].name(), j),
                &strategies[i],
            ));
        }
    }
    players
}
