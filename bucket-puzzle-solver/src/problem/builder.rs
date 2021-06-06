use super::bucket::Bucket;
use super::state::State;

pub struct Builder {
    buckets: Vec<Bucket>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            buckets: vec![],
        }
    }

    pub fn add_empty_bucket(mut self, capacity: u8) -> Self {
        self.buckets.push(Bucket::new(capacity));
        self
    }

    pub fn add_filled_bucket(mut self, capacity: u8) -> Self {
        self.buckets.push(Bucket::new(capacity).fill());
        self
    }

    pub fn build(self) -> State {
        State::new(self.buckets)
    }
}