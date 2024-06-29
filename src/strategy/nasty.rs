use crate::game::Move;
use crate::player::GamePlay;
use crate::strategy::Strategy;

pub struct Nasty {}

impl Strategy for Nasty {
    fn play(&self, _past_games: &[GamePlay]) -> Move {
        Move::Defect
    }
}
