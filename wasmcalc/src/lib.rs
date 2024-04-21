use wasm_bindgen::prelude::*;

mod utils;
mod calc;
pub use calc::Calc;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
