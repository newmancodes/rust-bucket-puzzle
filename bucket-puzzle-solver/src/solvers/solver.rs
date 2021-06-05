use crate::problem::state::State;
use super::solution::Solution;

pub trait Solver {
    fn solve(problem: &State) -> Option<Solution>;
}