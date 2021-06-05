use super::bucket::Bucket;

pub struct State {
    buckets: Vec<Bucket>,
    parent_state: Option<Box<State>>,
}

impl State {
    pub fn new() -> Self {
        State {
            buckets: vec![],
            parent_state: None,
        }
    }
}