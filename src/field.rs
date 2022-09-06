
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;


#[pyclass]
#[derive(Clone)]
pub struct Field{
    y_pos: i32,
    x_pos: i32,
    #[pyo3(get)]
    number: u8,
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
    current_color_id: u8
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
    pub fn set_number(&mut self, number: u8) -> PyResult<()> {
        if number < 10 {
            self.number = number;
            Ok(())
        } else {
            Err(PyValueError::new_err("number needs to be greater/equal than 0 and smaller than 10!"))
        }
    }

    #[setter]
    pub fn set_is_open(&mut self, is_open: bool) -> PyResult<()> {
        self.is_open = is_open;
        if is_open {
            self.current_color_id = self.number;
            if self.number == 0 {
                self.current_symbol = ' ';
            } else {
                let char_option = self.number.to_string().chars().nth(0);
                match char_option {
                    Some(value) => self.current_symbol = value,
                    None => return Err(PyValueError::new_err("number of Field is empty"))
                }
            }
        }
        Ok(())
    }

    #[setter]
    pub fn set_is_flag(&mut self, is_flag: bool) -> PyResult<()> {
        self.is_flag = is_flag;
        if is_flag {
            self.current_symbol = '?';
            self.current_color_id = 11;
            self.is_relevant = true;
        } else {
            self.current_symbol = '*';
            self.current_color_id = 0;
        }
        Ok(())
    }

    #[setter]
    pub fn set_is_relevant(&mut self, is_relevant: bool) -> PyResult<()> {
        if self.is_flag {
            return Ok(())
        }
        self.is_relevant = is_relevant;
        if is_relevant && !self.is_open {
            self.current_color_id = 0;
        } else if is_relevant && self.is_open {
            self.current_color_id = self.number;
        } else {
            self.current_color_id = 12;
        }
        Ok(())
    }
}