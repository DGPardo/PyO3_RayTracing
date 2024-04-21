from py_backend.py_math_lib.vector3 import Vector3


class Sphere:
    def __init__(self, center: Vector3, radius: float):
        self.center = center
        self.radius = radius

    def normal_at(self, point: Vector3):
        return (point - self.center).normalize()

    def __repr__(self):
        return f"Sphere({self.center}, {self.radius})"
