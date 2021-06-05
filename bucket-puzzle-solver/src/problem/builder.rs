use super::state::State;

pub struct Builder {
    state: State,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            state: State::new(),
        }
    }

    pub fn add_empty_bucket(&mut self, capacity: u8) -> &mut Self {
        self
    }

    pub fn build(self) -> State {
        self.state
    }
}