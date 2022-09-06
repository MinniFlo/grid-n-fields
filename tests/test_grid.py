import unittest
from grid_n_fields import Grid


class TestGrid(unittest.TestCase):

    def setUp(self):
        self.grid = Grid(12, 12, 0.2)

    def test_init(self):
        self.assertEqual(self.grid.y_size, 12)
        self.assertEqual(self.grid.x_size, 12)
        self.assertEqual(self.grid.field_count, 100)
        self.assertEqual(self.grid.mine_count, 18)
        self.assertEqual(self.grid.last_grid_state, [])

    def test_init_grid(self):
        field_0_0 = self.grid.grid[0][0]
        field_11_11 = self.grid.grid[11][11]
        field_7_3 = self.grid.grid[7][3]
        self.assertEqual(len(self.grid.grid), 12)
        self.assertEqual(len(self.grid.grid[0]), 12)
        self.assertEqual(field_0_0.coordinates, (0,0))
        self.assertEqual(field_11_11.coordinates, (11,11))
        self.assertEqual(field_7_3.coordinates, (7,3))

    def test_init_boarder(self):
        self.assertEqual(len(self.grid.boarder), 44)
        self.assertTrue((0, 0) in self.grid.boarder)
        self.assertTrue((0, 11) in self.grid.boarder)
        self.assertTrue((11, 0) in self.grid.boarder)
        self.assertTrue((11, 11) in self.grid.boarder)

    def test_get_field_with_coordinates(self):
        coordinates = (4, 5)
        field = self.grid.get_field_with_coordinates(coordinates)
        self.assertEqual(field.coordinates, coordinates)


if __name__ == '__main__':
    unittest.main()
