use crate::game::Move;
use crate::player::GamePlay;
use crate::strategy::Strategy;

pub struct GrimTrigger {}

impl Strategy for GrimTrigger {
    fn play(&self, _past_games: &[GamePlay]) -> Move {
        for game in _past_games {
            if game.their_move == Move::Defect {
                return Move::Defect;
            }
        }
        Move::Collaborate
    }
}
