use crate::game::Move;
use crate::player::GamePlay;

pub trait Strategy {
    fn play(&self, past_games: &[GamePlay]) -> Move;
    fn name(&self) -> String {
        format!("{}", std::any::type_name::<Self>())
    }
}

pub struct TitForTat {}

impl Strategy for TitForTat {
    fn play(&self, past_games: &[GamePlay]) -> Move {
        match past_games.last() {
            None => Move::Collaborate,
            Some(last_game) => last_game.their_move,
        }
    }
}

pub struct Naive {}

impl Strategy for Naive {
    fn play(&self, _past_games: &[GamePlay]) -> Move {
        Move::Collaborate
    }
}

pub struct Nasty {}

impl Strategy for Nasty {
    fn play(&self, _past_games: &[GamePlay]) -> Move {
        Move::Defect
    }
}

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

pub struct Drunk {}

impl Strategy for Drunk {
    fn play(&self, _past_games: &[GamePlay]) -> Move {
        match _past_games.last() {
            None => Move::Collaborate,
            Some(last_game) => last_game.my_move.oposite_move(),
        }
    }
}

pub fn all_strategies() -> Vec<Box<dyn Strategy>> {
    vec![
        Box::new(TitForTat {}),
        Box::new(Naive {}),
        Box::new(Nasty {}),
        Box::new(GrimTrigger {}),
        Box::new(Drunk {}),
    ]
}
