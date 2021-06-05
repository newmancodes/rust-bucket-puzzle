pub struct Rules {
    can_refill: bool,
    can_empty: bool,
}

impl Rules {
    pub fn new(can_refill: bool, can_empty: bool) -> Self {
        Rules {
            can_refill,
            can_empty,
        }
    }
}