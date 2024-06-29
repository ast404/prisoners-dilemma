use crate::game::Move;
use crate::player::GamePlay;
use crate::strategy::Strategy;

pub struct TitForTat {}

impl Strategy for TitForTat {
    fn play(&self, past_games: &[GamePlay]) -> Move {
        match past_games.last() {
            None => Move::Collaborate,
            Some(last_game) => last_game.their_move,
        }
    }
}
