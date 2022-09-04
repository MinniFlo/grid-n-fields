
use pyo3::prelude::*;


#[pyclass]
pub struct Field{
    #[pyo3(get, set)]
    number:i32
}

#[pymethods]
impl Field {
    #[new]
    pub fn new(number: i32) -> Self {
        Field{number: number}
    }
}