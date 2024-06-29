use crate::game::Move;
use crate::player::GamePlay;
use crate::strategy::Strategy;

pub struct Random {}

impl Strategy for Random {
    fn play(&self, _past_games: &[GamePlay]) -> Move {
        match rand::random::<bool>() {
            true => Move::Collaborate,
            false => Move::Defect,
        }
    }
}
