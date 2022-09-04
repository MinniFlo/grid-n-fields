
use pyo3::prelude::*;


#[pyclass]
pub struct Grid{
    #[pyo3(get, set)]
    number:i32
}

#[pymethods]
impl Grid {
    #[new]
    pub fn new(number: i32) -> Self {
        Grid{number: number}
    }
}