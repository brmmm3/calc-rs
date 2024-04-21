#[derive(Debug)]
pub struct Calc {
    result: f64,
}

impl Calc {
    pub fn new(value: f64) -> Self {
        Self { result: value }
    }

    pub fn add(&mut self, value: f64) -> f64 {
        self.result += value;
        self.result
    }

    pub fn sub(&mut self, value: f64) -> f64 {
        self.result -= value;
        self.result
    }

    pub fn result(&self) -> f64 {
        self.result
    }
}
