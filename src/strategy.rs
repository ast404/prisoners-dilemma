use crate::game::Move;
use crate::player::GamePlay;

pub mod drunk;
pub mod grim_trigger;
pub mod naive;
pub mod nasty;
pub mod random;
pub mod tit_for_tat;

pub trait Strategy {
    fn play(&self, past_games: &[GamePlay]) -> Move;
    fn name(&self) -> String {
        format!("{}", std::any::type_name::<Self>())
            .split("::")
            .last()
            .unwrap()
            .to_string()
    }
}

pub fn all_strategies() -> Vec<Box<dyn Strategy>> {
    vec![
        Box::new(tit_for_tat::TitForTat {}),
        Box::new(naive::Naive {}),
        Box::new(nasty::Nasty {}),
        Box::new(grim_trigger::GrimTrigger {}),
        Box::new(drunk::Drunk {}),
        Box::new(random::Random {}),
    ]
}
