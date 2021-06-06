use crate::problem::state::State;
use crate::problem::rules::Rules;
use super::solution::Solution;

pub trait Solver {
    fn solve(self, problem: State, rules: Rules, target: u8) -> Option<Solution>;
}