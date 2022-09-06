
use pyo3::prelude::*;
use std::collections::HashSet;
use crate::field::Field;


#[pyclass]
pub struct Grid{
    #[pyo3(get)]
    y_size:i32,
    #[pyo3(get)]
    x_size:i32,
    #[pyo3(get)]
    grid: Vec<Vec<Field>>,
    #[pyo3(get)]
    boarder: HashSet<(i32, i32)>,
    #[pyo3(get)]
    field_count: i32,
    #[pyo3(get)]
    mine_count: i32,
    #[pyo3(get)]
    last_grid_state: Vec<Vec<Field>>
}

#[pymethods]
impl Grid {
    #[new]
    pub fn new(y_size: i32, x_size: i32, percentage_of_mines: f32) -> Self {
        let grid = Self::build_grid(y_size, x_size);
        let boarder = Self::populate_boarder(y_size, x_size);
        let field_count = y_size * x_size - boarder.len() as i32;
        let mine_count = ((field_count - 9) as f32 * percentage_of_mines) as i32;

        Grid { 
            y_size: y_size, 
            x_size: x_size, 
            grid: grid, 
            boarder: boarder, 
            field_count: field_count, 
            mine_count: mine_count, 
            last_grid_state: Vec::new()
        }
    }
}

impl Grid {
    fn build_grid(y_size: i32, x_size: i32) -> Vec<Vec<Field>> {
        let mut grid: Vec<Vec<Field>> = vec![vec![]; y_size as usize];
        for y in 0..y_size as usize {
            for x in 0..x_size {
                let field = Field::new(y as i32, x);
                grid[y].push(field);
            }
        }
        grid
    }

    fn populate_boarder(y_size: i32, x_size: i32) -> HashSet<(i32, i32)> {
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
}