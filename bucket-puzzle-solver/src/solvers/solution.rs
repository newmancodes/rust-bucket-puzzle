use crate::problem::state::State;
pub struct Solution {
    pub steps: Vec<State>,
}

impl Solution {
    pub fn from(solved_state: State) -> Self {
        let mut steps = vec![];
        steps.push(solved_state);
        while let Some(parent_state) = solved_state.get_parent_state() {
            steps.push(parent_state.clone());
        }

        Solution {
            steps
        }
    }
}