use crate::game::Payoff;
use crate::player::{self, Player};

const DEFAULT_ITERATIONS: u32 = 10;

pub struct Tournament {
    payoff: Payoff,
    iterations: u32,
    play_twin: bool,
}

impl Tournament {
    pub fn all_pairs() -> Self {
        Self {
            payoff: Payoff::default(),
            iterations: DEFAULT_ITERATIONS,
            play_twin: false,
        }
    }

    pub fn axelrod_tournament() -> Self {
        Self {
            payoff: Payoff::default(),
            iterations: DEFAULT_ITERATIONS,
            play_twin: true,
        }
    }

    pub fn play_games(&self, players: &mut [Player]) {
        for i in 0..players.len() {
            let (left, right) = players.split_at_mut(i + 1);
            for j in 0..right.len() {
                player::play_games(&mut left[i], &mut right[j], &self.payoff, self.iterations);
            }
            if self.play_twin {
                let mut twin = left[i].twin();
                player::play_games(&mut left[i], &mut twin, &self.payoff, self.iterations)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::GamePlay;
    use crate::game::Move;
    use crate::strategy::Strategy;

    struct MockStrategy {
        next_move: Move,
    }

    impl Strategy for MockStrategy {
        fn play(&self, _past_games: &[GamePlay]) -> Move {
            self.next_move
        }
    }

    #[test]
    fn all_pairs_single_player() {
        let mock_strategy = MockStrategy {
            next_move: Move::Defect,
        };
        let tournament = Tournament::all_pairs();
        let mut players = vec![Player::new("single_player", &mock_strategy)];
        tournament.play_games(& mut players);
        assert_eq!(players[0].score(), 0);
    }

    #[test]
    fn all_pairs_two_players() {
        let collaborate_strategy = MockStrategy {
            next_move: Move::Collaborate,
        };
        let defect_strategy = MockStrategy {
            next_move: Move::Defect,
        };
        let tournament = Tournament::all_pairs();
        let mut players = vec![
            Player::new("collaborate_player", &collaborate_strategy),
            Player::new("defect_player", &defect_strategy),
        ];
        tournament.play_games(& mut players);
        assert_eq!(players[0].score(), 0);
        assert_eq!(players[1].score(), 50);
    }

    #[test]
    fn all_pairs_three_players() {
        let collaborate_strategy = MockStrategy {
            next_move: Move::Collaborate,
        };
        let defect_strategy = MockStrategy {
            next_move: Move::Defect,
        };
        let tournament = Tournament::all_pairs();
        let mut players = vec![
            Player::new("collaborate_player", &collaborate_strategy),
            Player::new("defect_player", &defect_strategy),
            Player::new("defect_player_2", &defect_strategy),
        ];
        tournament.play_games(& mut players);
        assert_eq!(players[0].score(), 0);
        assert_eq!(players[1].score(), 60);
        assert_eq!(players[2].score(), 60);
    }

    #[test]
    fn axelrod_single_player() {
        let mock_strategy = MockStrategy {
            next_move: Move::Defect,
        };
        let tournament = Tournament::axelrod_tournament();
        let mut players = vec![Player::new("single_player", &mock_strategy)];
        tournament.play_games(& mut players);
        assert_eq!(players[0].score(), 10);
    }

    #[test]
    fn axelrod_three_players() {
        let collaborate_strategy = MockStrategy {
            next_move: Move::Collaborate,
        };
        let defect_strategy = MockStrategy {
            next_move: Move::Defect,
        };
        let tournament = Tournament::axelrod_tournament();
        let mut players = vec![
            Player::new("collaborate_player", &collaborate_strategy),
            Player::new("defect_player", &defect_strategy),
            Player::new("defect_player_2", &defect_strategy),
        ];
        tournament.play_games(& mut players);
        assert_eq!(players[0].score(), 30);
        assert_eq!(players[1].score(), 70);
        assert_eq!(players[2].score(), 70);
    }
}
