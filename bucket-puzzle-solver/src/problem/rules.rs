#[derive(Copy, Clone)]
pub struct Rules {
    pub can_refill: bool,
    pub can_empty: bool,
}

impl Rules {
    pub fn new(can_refill: bool, can_empty: bool) -> Self {
        Rules {
            can_refill,
            can_empty,
        }
    }
}