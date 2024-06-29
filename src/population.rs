use crate::combinatorics;
use crate::player::Player;
use crate::strategy::Strategy;
use crate::tournament::Tournament;
use std::collections::HashMap;

pub fn simulate_populations(
    strategies: &[Box<dyn Strategy>],
    max_player_instances: u8,
    tournament: &Tournament,
) -> HashMap<String, u32> {
    let mut strategy_wins = HashMap::new();
    let all_combinations =
        combinatorics::get_combinations(strategies.len().try_into().unwrap(), max_player_instances);
    for player_counts in all_combinations {
        let mut players = create_players(strategies, &player_counts);
        let winning_strategy = get_winning_strategy(&mut players, tournament);
        strategy_wins
            .entry(winning_strategy)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    strategy_wins
}

fn get_winning_strategy(players: &mut [Player], tournament: &Tournament) -> String {
    tournament.play_games(players);
    let best_player = players.iter().max_by_key(|p| p.score());
    best_player.expect("at least one player").strategy_name()
}

fn create_players<'a>(
    strategies: &'a [Box<dyn Strategy>],
    player_counts: &'a [u8],
) -> Vec<Player<'a>> {
    assert_eq!(
        strategies.len(),
        player_counts.len(),
        "player counts don't match the size of strategies"
    );
    strategies
        .iter()
        .zip(player_counts.iter())
        .flat_map(|(strategy, count)| {
            std::iter::repeat(strategy)
                .take((*count + 1).into())
                .enumerate()
        })
        .map(|(i, strategy)| -> Player {
            Player::new(&format!("{}_{}", strategy.name(), i), strategy.as_ref())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::naive::Naive;
    use crate::strategy::nasty::Nasty;
    use crate::strategy::tit_for_tat::TitForTat;
    use crate::strategy::Strategy;

    #[test]
    fn naive_nasty() {
        let strategies: Vec<Box<dyn Strategy>> = vec![Box::new(Naive {}), Box::new(Nasty {})];
        let tournament = Tournament::all_pairs();
        let strategy_wins =
            simulate_populations(&strategies, /* max_player_instances= */ 4, &tournament);
        assert_eq!(strategy_wins.len(), 1);
        assert_eq!(*strategy_wins.get("Nasty").unwrap(), 16);
    }

    #[test]
    fn naive_nasty_tit_for_tat() {
        let strategies: Vec<Box<dyn Strategy>> = vec![
            Box::new(Naive {}),
            Box::new(Nasty {}),
            Box::new(TitForTat {}),
        ];
        let tournament = Tournament::all_pairs();
        let strategy_wins =
            simulate_populations(&strategies, /* max_player_instances= */ 4, &tournament);
        assert_eq!(strategy_wins.len(), 2);
        assert_eq!(*strategy_wins.get("Nasty").unwrap(), 52);
        assert_eq!(*strategy_wins.get("TitForTat").unwrap(), 12);
    }
}
