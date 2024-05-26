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
