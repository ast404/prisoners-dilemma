use crate::game::Move;
use crate::player::GamePlay;
use crate::strategy::Strategy;

pub struct Drunk {}

impl Strategy for Drunk {
    fn play(&self, _past_games: &[GamePlay]) -> Move {
        match _past_games.last() {
            None => Move::Collaborate,
            Some(last_game) => last_game.my_move.oposite_move(),
        }
    }
}
