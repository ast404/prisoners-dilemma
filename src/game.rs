#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Move {
    Collaborate,
    Defect,
}

impl Move {
    pub fn oposite_move(&self) -> Self {
        match *self {
            Move::Collaborate => Move::Defect,
            Move::Defect => Move::Collaborate,
        }
    }
}

pub struct Payoff {
    // From https://en.wikipedia.org/wiki/Prisoner%27s_dilemma#Generalized_form
    temptation_reward: u32,
    reward: u32,
    punishment: u32,
    suckers_punishment: u32,
}

impl Payoff {
    pub fn default() -> Self {
        Self::new(5, 3, 1, 0)
    }

    pub fn new(
        temptation_reward: u32,
        reward: u32,
        punishment: u32,
        suckers_punishment: u32,
    ) -> Self {
        assert!(temptation_reward > reward);
        assert!(reward > punishment);
        assert!(punishment > suckers_punishment);
        Self {
            temptation_reward,
            reward,
            punishment,
            suckers_punishment,
        }
    }

    pub fn compute_payoff(&self, p1_move: Move, p2_move: Move) -> (u32, u32) {
        match (p1_move, p2_move) {
            (Move::Collaborate, Move::Collaborate) => (self.reward, self.reward),
            (Move::Defect, Move::Collaborate) => (self.temptation_reward, self.suckers_punishment),
            (Move::Collaborate, Move::Defect) => (self.suckers_punishment, self.temptation_reward),
            (Move::Defect, Move::Defect) => (self.punishment, self.punishment),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_payoff() -> Payoff {
        Payoff::new(5, 3, 1, 0)
    }

    #[test]
    fn compute_payoff_both_collaborate() {
        let payoff = test_payoff();
        assert_eq!(
            payoff.compute_payoff(Move::Collaborate, Move::Collaborate),
            (3, 3)
        )
    }

    #[test]
    fn compute_payoff_both_defect() {
        let payoff = test_payoff();
        assert_eq!(payoff.compute_payoff(Move::Defect, Move::Defect), (1, 1))
    }

    #[test]
    fn compute_payoff_one_defect() {
        let payoff = test_payoff();
        assert_eq!(
            payoff.compute_payoff(Move::Defect, Move::Collaborate),
            (5, 0)
        )
    }

    #[test]
    fn compute_payoff_one_defects_simetric() {
        let payoff = test_payoff();
        let one_defects = payoff.compute_payoff(Move::Collaborate, Move::Defect);
        let other_defects = payoff.compute_payoff(Move::Defect, Move::Collaborate);
        assert_eq!(
            (one_defects.0, one_defects.1),
            (other_defects.1, other_defects.0)
        );
    }
}
