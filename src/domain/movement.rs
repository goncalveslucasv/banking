#[derive(Clone, Copy)]
pub struct Movement {
    amount: f32
}

impl Movement {
    pub fn create(amount: f32) -> Self {
        return Movement{amount}
    }

    pub fn obtain_amount(self) -> f32 {
        return self.amount
    }
}