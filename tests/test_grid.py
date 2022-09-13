
import unittest
from grid_n_fields import Grid

class FieldContext:

    def __init__(self, grid, coordinates):
        self.grid = grid
        self.coordinates = coordinates
        self.file = None

    def __enter__(self):
        self.field = self.grid.set_context_field(self.coordinates)
        return self.field

    def __exit__(self, exc_type, exc_value, exc_traceback):
        self.grid.update_context_field_in_grid(self.coordinates, self.field)
        return False

class TestGrid(unittest.TestCase):

    def setUp(self):
        self.grid = Grid(12, 12, 0.2)

    def test_init(self):
        self.assertEqual(self.grid.y_size, 12)
        self.assertEqual(self.grid.x_size, 12)
        self.assertEqual(self.grid.field_count, 100)
        self.assertEqual(self.grid.mine_count, 18)

    def test_init_grid(self):
        field_0_0 = self.grid.grid[0][0]
        field_11_11 = self.grid.grid[11][11]
        field_7_3 = self.grid.grid[7][3]
        self.assertEqual(len(self.grid.grid), 12)
        self.assertEqual(len(self.grid.grid[0]), 12)
        self.assertEqual(field_0_0.coordinates, (0,0))
        self.assertEqual(field_11_11.coordinates, (11,11))
        self.assertEqual(field_7_3.coordinates, (7,3))

        self.assertFalse(field_0_0.is_open)
        field_0_0.is_open = True
        self.assertTrue(field_0_0.is_open)

    def test_init_boarder(self):
        self.assertEqual(len(self.grid.boarder), 44)
        self.assertTrue((0, 0) in self.grid.boarder)
        self.assertTrue((0, 11) in self.grid.boarder)
        self.assertTrue((11, 0) in self.grid.boarder)
        self.assertTrue((11, 11) in self.grid.boarder)

    def test_neighbors_of_coordinates(self):
        neighbors = self.grid.neighbors_of_coordinates(1, 1)
        test_set = {(1, 2), (2, 1), (2, 2)}
        self.assertSetEqual(neighbors, test_set)
        neighbors = self.grid.neighbors_of_coordinates(3, 4)
        test_set = {(2, 3), (2, 4), (2, 5), (3, 3), (3, 5), (4, 3), (4, 4), (4, 5)}
        self.assertSetEqual(neighbors, test_set)

    def test_save_and_reset_gird(self):
        self.grid.update_last_grid()
    
    def test_active_field(self):
        coordinates = (4, 5)
        the_field = self.grid.set_context_field(coordinates)
        self.assertEqual(the_field.coordinates, coordinates)
        self.assertFalse(the_field.is_open)
        self.assertFalse(self.grid.grid[4][5].is_open)
        the_field.is_open = True
        self.grid.update_context_field_in_grid(coordinates, the_field)
        self.assertTrue(self.grid.grid[4][5].is_open)

    def test_field_context(self):
        coordinates = (6, 5)
        self.assertFalse(self.grid.grid[6][5].is_open)
        with FieldContext(self.grid, coordinates) as field:
            field.is_flag = True
        self.assertTrue(self.grid.grid[6][5].is_flag)
        self.assertEqual(self.grid.grid[6][5].symbol, '?')

    def test_get_field_with_coordinates_and_update(self):
        pass
        


    # wrapper functions
    # def test_set_is_open(self):
    #     self.assertFalse(self.grid.grid[2][3].is_open)
    #     self.grid.set_is_open((2,3), True)
    #     self.assertTrue(self.grid.grid[2][3].is_open)



if __name__ == '__main__':
    unittest.main()
