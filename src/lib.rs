
use pyo3::prelude::*;
pub mod field;
pub mod grid;


/// A Python module implemented in Rust.
#[pymodule]
fn grid_n_fields(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<field::Field>()?;
    m.add_class::<grid::Grid>()?;
    Ok(())
}