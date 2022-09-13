
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

    pub fn neighbors_of_coordinates(&self, coordinates: (usize, usize)) -> HashSet<(usize, usize)> {
        let (y, x) = coordinates;
        let neighbors = 
            vec![(y-1, x-1), (y-1, x), (y-1, x+1), (y, x-1), (y, x+1), (y+1, x-1), (y+1, x), (y+1, x+1)]
            .into_iter()
            .filter(|tup| !self.boarder.contains(tup))
            .collect();

        neighbors
    }

    // to access Fields in context manager
    pub fn set_context_field(&mut self, coordinates: (usize, usize)) -> Field {
        self.grid[coordinates.0][coordinates.1]
    }

    // to cleanup context manager
    pub fn update_context_field_in_grid(&mut self, coordinates: (usize, usize), field: Field) {
        self.grid[coordinates.0][coordinates.1] = field
    }

    // save game state
    pub fn update_last_grid(&mut self) {
        self.last_grid_state.clone_from_slice(&self.grid);
    }

    // reset game state
    pub fn reset_to_last_grid(&mut self) {
        self.grid.clone_from_slice(&self.last_grid_state);
    } 

    pub fn is_relevant_closed_field(&mut self, coordinates: (usize, usize)) -> bool {
        let neighbors = self.neighbors_of_coordinates(coordinates);
        for tuple in neighbors {
            let field = self.get_field_with_coordinates(tuple);
            if field.get_is_open() {
                return true;
            }
        }
        false
    }

    pub fn is_relevant_open_field(&mut self, coordinates: (usize, usize)) -> bool {
        let neighbors = self.neighbors_of_coordinates(coordinates);
        for tuple in neighbors {
            let field = self.get_field_with_coordinates(tuple);
            if !field.get_is_open() && !field.get_is_flag() {
                return true;
            }
        }
        false
    }
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