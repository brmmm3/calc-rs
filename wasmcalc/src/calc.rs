use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Calc {
    instance: calclib::Calc,
}

#[wasm_bindgen]
impl Calc {
    pub fn new() -> Self {
        Self {
            instance: calclib::Calc::new(0.0),
        }
    }

    pub fn add(&mut self, value: f64) -> f64 {
        self.instance.add(value)
    }

    pub fn sub(&mut self, value: f64) -> f64 {
        self.instance.sub(value)
    }

    pub fn result(&self) -> f64 {
        self.instance.result()
    }
}
