
use pyo3::prelude::*;


#[pyclass]
pub struct Field{
    y_pos: i32,
    x_pos: i32,
    #[pyo3(get, set)]
    number: i8,
    #[pyo3(get, set)]
    is_mine: bool,
    #[pyo3(get)]
    is_open: bool,
    #[pyo3(get)]
    is_flag: bool,
    #[pyo3(get)]
    is_relevant: bool,
    #[pyo3(get, set)]
    current_symbol: char,
    #[pyo3(get, set)]
    current_color_id: i32
}

#[pymethods]
impl Field {
    #[new]
    pub fn new(y_pos: i32, x_pos: i32) -> Self {
        Field{
            y_pos: y_pos,
            x_pos: x_pos,
            number: 0,
            is_mine: false,
            is_open: false,
            is_flag: false,
            is_relevant: false,
            current_symbol: '*',
            current_color_id: 12
        }
    }
    #[getter]
    pub fn coordinates(&self) -> PyResult<(i32, i32)> {
        Ok((self.y_pos, self.x_pos))
    }

    #[getter]
    pub fn render_coordinates(&self) -> PyResult<(i32, i32)> {
        Ok((self.y_pos, self.x_pos * 2))
    }

    #[setter]
    pub fn set_is_open(&mut self, is_open: bool) -> PyResult<()> {
        self.is_open = is_open;
        //todo
        Ok(())
    }

    #[setter]
    pub fn set_is_flag(&mut self, is_flag: bool) -> PyResult<()> {
        self.is_flag = is_flag;
        //todo
        Ok(())
    }

    #[setter]
    pub fn set_is_relevant(&mut self, is_relevant: bool) -> PyResult<()> {
        self.is_relevant = is_relevant;
        //todo
        Ok(())
    }
}