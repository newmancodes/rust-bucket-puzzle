use super::solution::Solution;
use super::solver::Solver;
use crate::problem::state::State;
use crate::problem::rules::Rules;

pub struct IterativeDeepeningSolver {
    max_depth: u8,
}

impl IterativeDeepeningSolver {
    pub fn new(max_depth: u8) -> Self {
        IterativeDeepeningSolver {
            max_depth
        }
    }
}

impl Solver for IterativeDeepeningSolver {
    fn solve(self, problem: &State, rules: &Rules, target: u8) -> Option<Solution> {
        None
    }
}