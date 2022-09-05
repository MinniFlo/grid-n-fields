import unittest
from grid_n_fields import Field


class TestField(unittest.TestCase):

    def setUp(self):
        self.field1 = Field(2, 4)

    def test_init(self):
        self.assertEqual(self.field1.coordinates, (2, 4))
        self.assertEqual(self.field1.number, 0)
        self.assertFalse(self.field1.is_mine)
        self.assertFalse(self.field1.is_open) 
        self.assertFalse(self.field1.is_flag) 
        self.assertFalse(self.field1.is_relevant)
        self.assertEqual(self.field1.current_symbol, '*')
        self.assertEqual(self.field1.current_color_id, 12)

    def test_render_coordinates(self):
        self.assertEqual(self.field1.render_coordinates, (2, 8))
        self.assertEqual(self.field2.render_coordinates, (6, 6))

    def test_set_number(self):
        self.field1.number = 5
        self.assertEqual(self.field1.number, 5)
        with self.assertRaises(ValueError):
            self.field1.number = 10

    def test_set_is_mine(self):
        self.field1.is_mine = True
        self.assertTrue(self.field1.is_mine)

    def test_set_current_symbol(self):
        self.field1.current_symbol = '1'
        self.assertEqual(self.field1.current_symbol, '1')


    def test_set_current_color_id(self):
        self.field1.current_color_id = 4
        self.assertEqual(self.field1.current_color_id, 4)

    def test_set_is_open(self):
        self.field1.number = 6
        self.field1.is_open = True
        self.assertTrue(self.field1.is_open)
        self.assertEqual(self.field1.current_symbol, '6')
        self.field1.is_open = False
        self.field1.number = 0
        self.field1.is_open = True
        self.assertTrue(self.field1.is_open)
        self.assertEqual(self.field1.current_symbol, ' ')

    def test_set_is_flag(self):
        self.field1.is_flag = True 
        self.assertTrue(self.field1.is_flag)
        self.assertEqual(self.field1.current_symbol, '?')
        self.assertEqual(self.field1.current_color_id, 11)
        self.assertTrue(self.field1.is_relevant)
        self.field1.is_flag = False
        self.assertFalse(self.field1.is_flag)
        self.assertEqual(self.field1.current_symbol, '*')
        self.assertEqual(self.field1.current_color_id, 0)
        self.assertTrue(self.field1.is_relevant)        

    def test_set_is_relevant(self):
        self.field1.is_open = True
        self.field1.number = 3
        self.field1.is_relevant = True
        self.assertTrue(self.field1.is_relevant)
        self.assertEqual(self.field1.current_color_id, 3)
        self.field1.is_relevant = False
        self.assertEqual(self.field1.current_color_id, 12)
        self.field1.is_open = False
        self.field1.is_relevant = True
        self.assertTrue(self.field1.is_relevant)
        self.assertEqual(self.field1.current_color_id, 0)
        self.field1.is_relevant = False
        self.assertEqual(self.field1.current_color_id, 12)


if __name__ == '__main__':
    unittest.main()
