
use pyo3::prelude::*;
use std::collections::HashSet;
use crate::field::Field;


#[pyclass]
#[derive(Clone)]
pub struct Grid{
    #[pyo3(get)]
    y_size: usize,
    #[pyo3(get)]
    x_size: usize,
    #[pyo3(get)]
    grid: Vec<Vec<Field>>,
    #[pyo3(get)]
    boarder: HashSet<(usize, usize)>,
    #[pyo3(get)]
    field_count: usize,
    #[pyo3(get)]
    mine_count: usize,
    #[pyo3(get)]
    last_grid_state: Vec<Vec<Field>>,
}

#[pymethods]
impl Grid {
    #[new]
    pub fn new(y_size: usize, x_size: usize, percentage_of_mines: f32) -> Self {
        let grid = Self::build_grid(y_size, x_size);
        let last_grid_state = grid.clone();
        let boarder = Self::populate_boarder(y_size, x_size);
        let field_count = y_size * x_size - boarder.len() as usize;
        let mine_count = ((field_count - 9) as f32 * percentage_of_mines) as usize;

        Grid { 
            y_size: y_size, 
            x_size: x_size, 
            grid: grid, 
            boarder: boarder, 
            field_count: field_count, 
            mine_count: mine_count, 
            last_grid_state: last_grid_state,
        }
    }

    pub fn neighbors_of_coordinates(&self, y: usize, x: usize) -> HashSet<(usize, usize)> {
        let neighbors = 
            vec![(y-1, x-1), (y-1, x), (y-1, x+1), (y, x-1), (y, x+1), (y+1, x-1), (y+1, x), (y+1, x+1)]
            .into_iter()
            .filter(|tup| !self.boarder.contains(tup))
            .collect();

        neighbors
    }

    pub fn set_context_field(&mut self, coordinates: (usize, usize)) -> Field {
        self.grid[coordinates.0][coordinates.1]
    }

    pub fn update_context_field_in_grid(&mut self, coordinates: (usize, usize), field: Field) {
        self.grid[coordinates.0][coordinates.1] = field
    }

    pub fn update_last_grid(&mut self) {
        self.last_grid_state.clone_from_slice(&self.grid);
    }

    pub fn reset_to_last_grid(&mut self) {
        self.grid.clone_from_slice(&self.last_grid_state);
    }

    // //wrapper functions for Field getter and setter

    // pub fn get_render_coordinates(&mut self, coordinates: (usize, usize)) -> PyResult<(usize, usize)> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.render_coordinates()
    // }

    // //number
    // pub fn get_number(&mut self, coordinates: (usize, usize)) -> PyResult<u8> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.get_number()
    // }

    // pub fn set_number(&mut self, coordinates: (usize, usize), number: u8) -> PyResult<()> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.set_number(number)
    // }

    // //is_mine
    // pub fn get_is_mine(&mut self, coordinates: (usize, usize)) -> PyResult<bool> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.get_is_mine()
    // }

    // pub fn set_is_mine(&mut self, coordinates: (usize, usize), is_mine: bool) -> PyResult<()> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.set_is_mine(is_mine)?;
    //     Ok(())
    // }

    // //is_open
    // pub fn get_is_open(&mut self, coordinates: (usize, usize)) -> PyResult<bool> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.get_is_open()
    // }

    // pub fn set_is_open(&mut self, coordinates: (usize, usize), is_open: bool) -> PyResult<()> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.set_is_open(is_open)?;
    //     Ok(())
    // }

    // //is_flag
    // pub fn get_is_flag(&mut self, coordinates: (usize, usize)) -> PyResult<bool> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.get_is_flag()
    // }

    // pub fn set_is_flag(&mut self, coordinates: (usize, usize), is_flag: bool) -> PyResult<()> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.set_is_flag(is_flag)?;
    //     Ok(())
    // }

    // //is_relevant
    // pub fn set_is_relevant(&mut self, coordinates: (usize, usize), is_relevant: bool) -> PyResult<()> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.set_is_relevant(is_relevant)?;
    //     Ok(())
    // }

    // //symbol
    // pub fn get_get_symbol(&mut self, coordinates: (usize, usize)) -> PyResult<char> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.get_symbol()
    // }
    
    // //color_id
    // pub fn get_get_color_id(&mut self, coordinates: (usize, usize)) -> PyResult<u8> {
    //     let field: &mut Field = self.get_field_with_coordinates(coordinates);
    //     field.get_color_id()
    // } 
}

impl Grid {
    fn build_grid(y_size: usize, x_size: usize) -> Vec<Vec<Field>> {
        let mut grid: Vec<Vec<Field>> = vec![vec![]; y_size as usize];
        for y in 0..y_size {
            for x in 0..x_size {
                let field = Field::new(y, x);
                grid[y].push(field);
            }
        }
        grid
    }

    fn populate_boarder(y_size: usize, x_size: usize) -> HashSet<(usize, usize)> {
        let mut boarder = HashSet::new();
        for y in 0..y_size {
            boarder.insert((y, 0));
            boarder.insert((y, x_size - 1));
        }
        for x in 0..x_size {
            boarder.insert((0, x));
            boarder.insert((y_size - 1, x));
        }
        boarder
    }

    pub fn get_field_with_coordinates(&mut self, coordinates: (usize, usize)) -> &mut Field {
        &mut self.grid[coordinates.0][coordinates.1]
    }
}