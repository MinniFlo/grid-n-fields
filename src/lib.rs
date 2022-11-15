use pyo3::prelude::*;
use rand::prelude::*;

pub mod field;
pub mod grid;

#[pyfunction]
pub fn generate_mine_positions(
    mut possible_mine_coordinates: Vec<(usize, usize)>,
    mine_count: usize,
) -> Vec<(usize, usize)> {
    let mut mine_positions = Vec::new();
    let possible_mine_count = possible_mine_coordinates.len();
    let mut rng = thread_rng();
    for n in 0..mine_count {
        let random_index = rng.gen_range(0..possible_mine_count - n);
        let mine_coordinate = possible_mine_coordinates.remove(random_index);
        mine_positions.push(mine_coordinate);
    }

    mine_positions
}

/// A Python module implemented in Rust.
#[pymodule]
fn grid_n_fields(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<field::Field>()?;
    m.add_class::<grid::Grid>()?;
    m.add_function(wrap_pyfunction!(generate_mine_positions, m)?)?;
    Ok(())
}
