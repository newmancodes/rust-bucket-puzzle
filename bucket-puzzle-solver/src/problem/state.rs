use super::bucket::Bucket;
use crate::problem::rules::Rules;

pub struct State {
    buckets: Vec<Bucket>,
    parent_state: Option<Box<State>>,
}

impl State {
    pub fn new(buckets: Vec<Bucket>) -> Self {
        State {
            buckets,
            parent_state: None,
        }
    }

    pub fn is_a_solution(&self, target: u8) -> bool {
        for bucket in &self.buckets {
            if bucket.get_current_volume() == target {
                return true;
            }
        }

        false
    }

    pub fn generate_child_states(&self, rules: &Rules) -> Vec<State>
    {
        vec![]
    }
}