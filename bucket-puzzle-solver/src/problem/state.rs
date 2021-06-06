use super::bucket::Bucket;
use crate::problem::rules::Rules;
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

#[derive(Clone)]
pub struct State {
    buckets: Vec<Bucket>,
    parent_state: Option<Rc<State>>,
}

impl PartialEq<State> for State {
    fn eq(&self, other: &State) -> bool {
        self.buckets == other.buckets
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("State")
            .field("buckets", &self.buckets)
            .field("parent_state", &self.parent_state)
            .finish()
    }
}

impl State {
    pub fn new(buckets: Vec<Bucket>) -> Self {
        State {
            buckets,
            parent_state: None,
        }
    }

    pub fn get_parent_state(&self) -> Option<&State> {
        if self.parent_state.is_some() {
            return Some(self.parent_state.unwrap().clone().borrow());
        }

        None
    }

    pub fn with_parent_state(parent_state: Rc<State>, buckets: Vec<Bucket>) -> Self {
        State {
           buckets,
            parent_state: Some(parent_state),
        }
    }

    pub fn is_a_solution(&self, target: u8) -> bool {
        while let Some(bucket) = self.buckets.iter().next() {
            if bucket.get_current_volume() == target {
                return true;
            }
        }

        false
    }

    pub fn generate_child_states(self, rules: Rules) -> Vec<State>
    {
        let mut child_states = vec![];
        let rc_self = Rc::new(self);

        while let Some(bucket) = rc_self.buckets.iter().next() {
            let mut new_buckets = vec![];

            if !bucket.is_empty() {
                if rules.can_empty {
                    // empty the bucket
                    new_buckets.push(bucket.empty());
                }

                while let Some(other_bucket) = rc_self.buckets.iter().next() {
                    if bucket != other_bucket {
                        // pour from bucket into other_bucket
                        let poured_buckets = bucket.pour_into(*other_bucket);
                        new_buckets.push(poured_buckets.0);
                        new_buckets.push(poured_buckets.1);
                    }
                }
            }

            if rules.can_refill && !bucket.is_full() {
                // fill the bucket - and add the new state
                new_buckets.push(bucket.fill());
            }

            child_states.push(State::with_parent_state(rc_self.clone(), new_buckets));
        }

        child_states
    }
}