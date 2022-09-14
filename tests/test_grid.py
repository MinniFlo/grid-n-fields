
import unittest
from grid_n_fields import FieldContext, Grid

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
        neighbors = self.grid.neighbors_of_coordinates((1, 1))
        test_set = {(1, 2), (2, 1), (2, 2)}
        self.assertSetEqual(neighbors, test_set)
        neighbors = self.grid.neighbors_of_coordinates((3, 4))
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
        self.grid.update_last_grid()
        coordinates = (6, 5)
        with FieldContext(self.grid, coordinates) as field:
            field.is_flag = True
        self.assertTrue(self.grid.grid[6][5].is_flag)
        self.grid.reset_to_last_grid()
        self.assertFalse(self.grid.grid[6][5].is_flag)

    def test_is_relevant_field(self):
        with FieldContext(self.grid, (4, 4)) as field:
            field.is_open = True
        with FieldContext(self.grid, (4, 3)) as field:
            field.is_open = True
        with FieldContext(self.grid, (4, 2)) as field:
            field.is_open = True
        with FieldContext(self.grid, (5, 4)) as field:
            field.is_open = True
        with FieldContext(self.grid, (5, 3)) as field:
            field.is_open = True
        with FieldContext(self.grid, (5, 2)) as field:
            field.is_open = True
        with FieldContext(self.grid, (3, 4)) as field:
            field.is_open = True
        with FieldContext(self.grid, (3, 3)) as field:
            field.is_open = True
        with FieldContext(self.grid, (3, 2)) as field:
            field.is_open = True
        with FieldContext(self.grid, (6, 2)) as field:
            field.is_flag = True
        with FieldContext(self.grid, (6, 3)) as field:
            field.is_flag = True
        with FieldContext(self.grid, (6, 4)) as field:
            field.is_flag = True

        #  1 2 3 4 5 6 
        #  2 * * * * * 
        #  3 . . . * * 
        #  4 . . . * * 
        #  5 . . . * * 
        #  6 ? ? ? * * 

        self.assertTrue(self.grid.is_relevant_closed_field((5, 5)))
        self.assertTrue(self.grid.is_relevant_closed_field((4, 5)))
        self.assertTrue(self.grid.is_relevant_closed_field((6, 5)))
        self.assertFalse(self.grid.is_relevant_closed_field((4, 6)))

        self.assertTrue(self.grid.is_relevant_open_field((4, 4)))
        self.assertTrue(self.grid.is_relevant_open_field((5, 4)))
        self.assertFalse(self.grid.is_relevant_open_field((4, 3)))
        self.assertFalse(self.grid.is_relevant_open_field((5, 3)))

    def test_set_mine_data(self):
        y, x = (2, 2)
        self.grid.set_mine_data((y, x))

        for row in self.grid.grid:
            print("\n")
            for field in row:
                coordinates = field.coordinates
                if field.is_mine:
                    print("* ", end=' ')
                else:
                    if coordinates in self.grid.boarder:
                        print("# ", end=' ')
                    elif field.number == 0:
                         print("  ", end=' ')
                    else: 
                         print("{} ".format(field.number), end=' ')

        mine_counter = 0
        start_fields = {(y+1, x+1),(y+1, x), (y+1, x-1), (y, x+1), (y, x), (y, x-1), (y-1, x+1), (y-1, x), (y-1, x-1)}
        possitions = set()
        for y in range(self.grid.y_size):
            for x in range(self.grid.x_size):
                with FieldContext(self.grid, (y, x)) as field:
                    coordinates = field.coordinates
                    if field.is_mine:
                        mine_counter += 1
                        possitions.add(coordinates)
                    else: 
                        if coordinates in self.grid.boarder:
                            continue
                        neighbors = self.grid.neighbors_of_coordinates(coordinates)
                        number = 0
                        for tupl in neighbors:
                            with FieldContext(self.grid, tupl) as f:
                                if f.is_mine:
                                    number += 1
                        self.assertEqual(field.number, number)
    
        self.assertEqual(mine_counter, self.grid.mine_count)
        for i in start_fields:
            self.assertFalse(i in possitions)
            self.assertFalse(i in self.grid.boarder)
    




if __name__ == '__main__':
    unittest.main()
