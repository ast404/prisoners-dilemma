use crate::game::Move;
use crate::player::GamePlay;
use crate::strategy::Strategy;

pub struct Naive {}

impl Strategy for Naive {
    fn play(&self, _past_games: &[GamePlay]) -> Move {
        Move::Collaborate
    }
}
