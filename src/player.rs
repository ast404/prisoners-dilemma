use crate::game::{Move, Payoff};
use crate::strategy::Strategy;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct GamePlay {
    pub my_move: Move,
    pub their_move: Move,
}

pub struct Player<'a> {
    pub name: String,
    score: u32,
    past_games: HashMap<String, Vec<GamePlay>>,
    strategy: &'a dyn Strategy,
}

impl<'a> Player<'a> {
    pub fn new(name: &str, strategy: &'a dyn Strategy) -> Self {
        Self {
            name: name.to_string(),
            score: 0,
            past_games: HashMap::new(),
            strategy,
        }
    }

    pub fn twin(&self) -> Self {
        Self::new(&format!("{}_twin", self.name), self.strategy)
    }

    fn play(&mut self, with_player: &str) -> Move {
        let past_games = self
            .past_games
            .entry(with_player.to_string())
            .or_insert_with(Vec::new);
        self.strategy.play(past_games)
    }

    fn name(&self) -> &str {
        &self.name
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn strategy_name(&self) -> String {
        self.strategy.name()
    }

    fn game_played(&mut self, with_player: &str, my_move: Move, their_move: Move, points: u32) {
        self.past_games
            .entry(with_player.to_string())
            .or_insert_with(Vec::new)
            .push(GamePlay {
                my_move: my_move,
                their_move: their_move,
            });
        self.score += points;
    }
}

pub fn play_games(p1: &mut Player, p2: &mut Player, payoff: &Payoff, games_num: u32) {
    for _ in 0..games_num {
        play_game(p1, p2, payoff);
    }
}

fn play_game(p1: &mut Player, p2: &mut Player, payoff: &Payoff) {
    let p1_move = p1.play(&p2.name());
    let p2_move = p2.play(&p1.name());
    let score = payoff.compute_payoff(p1_move, p2_move);
    p1.game_played(&p2.name(), p1_move, p2_move, score.0);
    p2.game_played(&p1.name(), p2_move, p1_move, score.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct MockStrategy {
        next_move: Cell<Move>,
    }

    impl Strategy for MockStrategy {
        fn play(&self, _past_games: &[GamePlay]) -> Move {
            self.next_move.get()
        }
    }

    fn test_payoff() -> Payoff {
        Payoff::new(5, 3, 1, 0)
    }

    #[test]
    fn player_and_stragety_name() {
        let mock_strategy = MockStrategy {
            next_move: Cell::new(Move::Defect),
        };
        let player = Player::new("test_player", &mock_strategy);
        assert_eq!(player.name(), "test_player");
        assert_eq!(player.strategy_name(), "MockStrategy");
    }

    #[test]
    fn twin() {
        let mock_strategy = MockStrategy {
            next_move: Cell::new(Move::Defect),
        };
        let player = Player::new("test_player", &mock_strategy);
        let twin = player.twin();
        assert_eq!(twin.name(), "test_player_twin");
        assert_eq!(twin.strategy_name(), "MockStrategy");
        assert_eq!(player.strategy_name(), "MockStrategy");
    }

    #[test]
    fn one_game_played() {
        let defect_strategy = MockStrategy {
            next_move: Cell::new(Move::Defect),
        };
        let mut defect_player = Player::new("defect_player", &defect_strategy);
        let collaborate_strategy = MockStrategy {
            next_move: Cell::new(Move::Collaborate),
        };
        let mut collaborate_player = Player::new("collaborate_player", &collaborate_strategy);
        assert_eq!(defect_player.score, 0);
        assert_eq!(collaborate_player.score, 0);
        play_game(&mut defect_player, &mut collaborate_player, &test_payoff());
        assert_eq!(defect_player.score, 5);
        assert_eq!(
            *defect_player.past_games.get("collaborate_player").unwrap(),
            vec![GamePlay {
                my_move: Move::Defect,
                their_move: Move::Collaborate
            }]
        );
        assert_eq!(collaborate_player.score, 0);
        assert_eq!(
            *collaborate_player.past_games.get("defect_player").unwrap(),
            vec![GamePlay {
                my_move: Move::Collaborate,
                their_move: Move::Defect
            }]
        );
    }

    #[test]
    fn consecutive_games_played() {
        let defect_strategy = MockStrategy {
            next_move: Cell::new(Move::Defect),
        };
        let mut defect_player = Player::new("defect_player", &defect_strategy);
        let alternate_strategy = MockStrategy {
            next_move: Cell::new(Move::Collaborate),
        };
        let mut alternate_player = Player::new("alternate_player", &alternate_strategy);
        assert_eq!(defect_player.score, 0);
        assert_eq!(alternate_player.score, 0);
        play_game(&mut defect_player, &mut alternate_player, &test_payoff());
        assert_eq!(defect_player.score, 5);
        assert_eq!(alternate_player.score, 0);
        alternate_strategy.next_move.replace(Move::Defect);
        play_game(&mut defect_player, &mut alternate_player, &test_payoff());
        assert_eq!(defect_player.score, 6);
        assert_eq!(
            *defect_player.past_games.get("alternate_player").unwrap(),
            vec![
                GamePlay {
                    my_move: Move::Defect,
                    their_move: Move::Collaborate
                },
                GamePlay {
                    my_move: Move::Defect,
                    their_move: Move::Defect
                }
            ]
        );
        assert_eq!(alternate_player.score, 1);
        assert_eq!(
            *alternate_player.past_games.get("defect_player").unwrap(),
            vec![
                GamePlay {
                    my_move: Move::Collaborate,
                    their_move: Move::Defect
                },
                GamePlay {
                    my_move: Move::Defect,
                    their_move: Move::Defect
                }
            ]
        );
    }

    #[test]
    fn games_played() {
        let defect_strategy = MockStrategy {
            next_move: Cell::new(Move::Defect),
        };
        let mut defect_player = Player::new("defect_player", &defect_strategy);
        let collaborate_strategy = MockStrategy {
            next_move: Cell::new(Move::Collaborate),
        };
        let mut collaborate_player = Player::new("collaborate_player", &collaborate_strategy);
        assert_eq!(defect_player.score, 0);
        assert_eq!(collaborate_player.score, 0);
        play_games(
            &mut defect_player,
            &mut collaborate_player,
            &test_payoff(),
            9,
        );
        assert_eq!(defect_player.score, 45);
        assert_eq!(
            defect_player
                .past_games
                .get("collaborate_player")
                .unwrap()
                .len(),
            9
        );
        assert_eq!(collaborate_player.score, 0);
        assert_eq!(
            collaborate_player
                .past_games
                .get("defect_player")
                .unwrap()
                .len(),
            9
        );
    }
}
