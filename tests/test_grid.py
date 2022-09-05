import unittest
from grid_n_fields import Field, Grid


class TestGrid(unittest.TestCase):

    def setUp(self):
        self.grid = Grid(1)

    def test_init(self):
        self.assertTrue(self.grid)



if __name__ == '__main__':
    unittest.main()
