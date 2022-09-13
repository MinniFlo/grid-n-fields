use crate::field::Field;
use Grid;

#[pymethods]
impl Grid {
    // coordinates
    pub fn get_coordinates(&self, coordinates: (usize, usize)) -> PyResult<(usize, usize)> {
        let field: &mut Field = self.get_field_with_coordinates(coordinates);
        field.coordinates()
    }

    pub fn set_is_open(&self, coordinates: (usize, usize), is_open: bool) -> PyResult<()> {
        let field: &mut Field = self.get_field_with_coordinates(coordinates);
        field.set_is_open(is_open);
        Ok(())
    }


}