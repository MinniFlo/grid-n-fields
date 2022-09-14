
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