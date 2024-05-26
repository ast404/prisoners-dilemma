use crate::game::{Move, Payoff};
use crate::strategy::Strategy;
use std::collections::HashMap;

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
