use super::solution::Solution;
use super::solver::Solver;
use crate::problem::state::State;
use crate::problem::rules::Rules;

pub struct IterativeDeepeningSolver {
    max_depth: u8,
}

#[derive(Clone)]
struct StateWithDepth {
    pub state: State,
    pub depth: u8,
}

impl StateWithDepth {
    pub fn new(state: State, depth: u8) -> Self {
        StateWithDepth {
            state,
            depth,
        }
    }
}

impl PartialEq<State> for StateWithDepth {
    fn eq(&self, other: &State) -> bool {
        self.state == *other
    }
}

impl PartialEq<StateWithDepth> for State {
    fn eq(&self, other: &StateWithDepth) -> bool {
        *self == other.state
    }
}

impl IterativeDeepeningSolver {
    pub fn new(max_depth: u8) -> Self {
        IterativeDeepeningSolver {
            max_depth
        }
    }

    fn solve_with_limit(&self, state: &State, rules: Rules, target: u8, limit: u8) -> Option<Solution> {
        let mut frontier = vec![];
        let mut explored = vec![];

        frontier.push(StateWithDepth::new(state.clone(), 0));

        loop {
            let candidate = frontier.pop().unwrap().clone();
            if candidate.state.is_a_solution(target) {
                return Some(Solution::from(candidate.state));
            }

            explored.push(candidate.clone());

            if candidate.depth < limit {
                let child_states = candidate.state.generate_child_states(rules);
                for child_state in child_states {
                    if !explored.iter().any(| explored_state | { *explored_state == child_state})
                        && !frontier.iter().any(| frontier_state | { *frontier_state == child_state}) {
                        frontier.push(StateWithDepth::new(child_state, candidate.depth + 1));
                    }
                }
            }

            if frontier.len() == 0 {
                break;
            }
        }

        None
    }
}

impl Solver for IterativeDeepeningSolver {
    fn solve(self, problem: State, rules: Rules, target: u8) -> Option<Solution> {
        let mut count: u8 = 0;

        loop {
            count += 1;

            let result = self.solve_with_limit(&problem, rules, target, count);

            if result.is_some() {
                return result;
            }

            if count == self.max_depth {
                break;
            }
        }

        None
    }
}