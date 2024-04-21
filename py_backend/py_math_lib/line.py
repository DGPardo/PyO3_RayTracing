from py_backend.py_math_lib.vector3 import Vector3


class Line:
    def __init__(self, origin: Vector3, direction: Vector3):
        self.origin = origin
        self.direction = direction.normalize()

    def __repr__(self):
        return f"Line({self.origin}, {self.direction})"
