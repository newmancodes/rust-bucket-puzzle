use crate::problem::state::State;
use std::rc::Rc;

pub struct Solution {
    pub steps: Vec<Rc<State>>,
}

impl Solution {
    pub fn from(solved_state: State) -> Self {
        let mut steps = vec![];
        steps.push(Rc::new(solved_state.clone()));
        while let Some(parent_state) = solved_state.get_parent_state() {
            steps.push(parent_state.clone());
        }

        Solution {
            steps
        }
    }
}