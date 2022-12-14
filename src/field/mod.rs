use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy)]
pub struct Field {
    y_pos: usize,
    x_pos: usize,
    #[pyo3(get)]
    number: u8,
    #[pyo3(get)]
    is_mine: bool,
    #[pyo3(get)]
    is_open: bool,
    #[pyo3(get)]
    is_flag: bool,
    #[pyo3(get)]
    is_relevant: bool,
    #[pyo3(get)]
    symbol: char,
    #[pyo3(get)]
    color_id: u8,
}

#[pymethods]
impl Field {
    #[new]
    pub fn new(y_pos: usize, x_pos: usize) -> Self {
        Field {
            y_pos: y_pos,
            x_pos: x_pos,
            number: 0,
            is_mine: false,
            is_open: false,
            is_flag: false,
            is_relevant: false,
            symbol: '*',
            color_id: 12,
        }
    }
    #[getter]
    pub fn coordinates(&self) -> PyResult<(usize, usize)> {
        Ok((self.y_pos, self.x_pos))
    }

    #[getter]
    pub fn render_coordinates(&self) -> PyResult<(usize, usize)> {
        Ok((self.y_pos, self.x_pos * 2))
    }

    #[setter]
    pub fn set_is_open(&mut self, is_open: bool) -> PyResult<()> {
        self.is_open = is_open;
        if is_open {
            self.color_id = self.number;
            if self.number == 0 {
                self.symbol = ' ';
            } else {
                let char_option = self.number.to_string().chars().nth(0);
                match char_option {
                    Some(value) => self.symbol = value,
                    None => return Err(PyValueError::new_err("number of Field is empty")),
                }
            }
        }
        Ok(())
    }

    #[setter]
    pub fn set_is_flag(&mut self, is_flag: bool) -> PyResult<()> {
        self.is_flag = is_flag;
        if is_flag {
            self.symbol = '?';
            self.color_id = 11;
            self.is_relevant = true;
        } else {
            self.symbol = '*';
            self.color_id = 0;
        }
        Ok(())
    }

    #[setter]
    pub fn set_is_relevant(&mut self, is_relevant: bool) -> PyResult<()> {
        if self.is_flag {
            return Ok(());
        }
        self.is_relevant = is_relevant;
        if is_relevant && !self.is_open {
            self.color_id = 0;
        } else if is_relevant && self.is_open {
            self.color_id = self.number;
        } else {
            self.color_id = 12;
        }
        Ok(())
    }
}

impl Field {
    pub fn get_coordinates(&self) -> (usize, usize) {
        (self.y_pos, self.x_pos)
    }

    pub fn get_is_open(&self) -> bool {
        self.is_open
    }

    pub fn get_is_flag(&self) -> bool {
        self.is_flag
    }

    pub fn get_is_mine(&self) -> bool {
        self.is_mine
    }

    pub fn set_is_mine(&mut self, is_mine: bool) {
        self.is_mine = is_mine;
    }

    pub fn set_number(&mut self, number: u8) {
        self.number = number;
    }
}
