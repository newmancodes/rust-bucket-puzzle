use super::bucket::Bucket;

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
}