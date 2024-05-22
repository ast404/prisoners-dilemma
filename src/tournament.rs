use crate::game::Payoff;
use crate::player::{self, Player};

const DEFAULT_ITERATIONS: u32 = 10;

pub struct AllPairsTournament {
    payoff: Payoff,
    iterations: u32,
}

impl AllPairsTournament {
    pub fn new() -> Self {
        Self {
            payoff: Payoff::default(),
            iterations: DEFAULT_ITERATIONS,
        }
    }

    pub fn play_games(&self, players: &mut [Player]) {
        for i in 0..players.len() {
            let (left, right) = players.split_at_mut(i + 1);
            for j in 0..right.len() {
                for _ in 0..self.iterations {
                    player::play_game(&mut left[i], &mut right[j], &self.payoff)
                }
            }
        }
    }
}
