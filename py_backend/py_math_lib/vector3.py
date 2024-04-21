import math
from py_backend.py_math_lib.constants import VSMALL


class Vector3:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def norm(self):
        return math.sqrt(self.x**2 + self.y**2 + self.z**2)

    def scale(self, s) -> "Vector3":
        return Vector3(self.x * s, self.y * s, self.z * s)

    def dot(self, other: "Vector3") -> float:
        return self.x * other.x + self.y * other.y + self.z * other.z

    def normalize(self) -> "Vector3":
        return self.scale(1 / max(self.norm(), VSMALL))

    def __add__(self, other: "Vector3") -> "Vector3":
        return Vector3(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other: "Vector3") -> "Vector3":
        return Vector3(self.x - other.x, self.y - other.y, self.z - other.z)

    def __repr__(self):
        return f"Vector3({self.x}, {self.y}, {self.z})"
