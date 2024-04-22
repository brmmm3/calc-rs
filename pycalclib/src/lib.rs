use pyo3::prelude::*;

mod calc;

/// pycalclib_rs is a simple example for using Rust to create Python extension modules.
#[pymodule]
#[pyo3(name = "pycalclib_rs")]
fn init(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<calc::Calc>()?;
    Ok(())
}
