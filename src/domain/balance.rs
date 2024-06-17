#[derive(Clone)]
pub struct Balance {
    amount: f32
}

impl Balance {
    pub fn create() -> Self {
        Self { amount: 0f32 }
    }
    pub fn increase(&self, amount: f32) -> Balance {
        return Self { amount: amount + self.amount}
    }

    pub fn decrease(&self, amount: f32) -> Balance {
        return Self { amount: amount - self.amount}
    }

    pub fn amount(&self) -> f32 {
        self.amount
    }
}