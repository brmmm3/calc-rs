use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Calc {
    instance: calclib::Calc,
}

#[pymethods]
impl Calc {
    #[new]
    fn new(value: Option<f64>) -> Self {
        Self {
            instance: calclib::Calc::new(value.unwrap_or(0.0)),
        }
    }

    pub fn add(&mut self, value: f64) -> f64 {
        self.instance.add(value)
    }

    pub fn sub(&mut self, value: f64) -> f64 {
        self.instance.sub(value)
    }

    #[getter]
    pub fn result(&self) -> f64 {
        self.instance.result()
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}
