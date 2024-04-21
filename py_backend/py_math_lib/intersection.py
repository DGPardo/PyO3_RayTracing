from typing import Union, Tuple

from py_backend.py_math_lib.vector3 import Vector3
from py_backend.py_math_lib.sphere import Sphere
from py_backend.py_math_lib.line import Line
from py_backend.py_math_lib.constants import VSMALL


def intersect_line_sphere(
    line: Line, sphere: Sphere
) -> Union[None, Tuple[Vector3, Vector3]]:
    """
    Calculate the intersection of a line and a sphere.
    There are 3 possible outcomes:
        1. The line intersects the sphere at 2 points.
        2. The line intersects the sphere at 1 point.
        3. The line does not intersect the sphere.
    """
    oc = line.origin - sphere.center
    a = line.direction.dot(line.direction)
    b = 2.0 * oc.dot(line.direction)
    c = oc.dot(oc) - sphere.radius**2
    discriminant = b**2.0 - 4.0 * a * c
    if discriminant < 0:
        # No intersection
        return None
    t1 = (-b + discriminant**0.5) / max((2.0 * a), VSMALL)
    t2 = (-b - discriminant**0.5) / max((2.0 * a), VSMALL)

    points = (
        line.origin + line.direction.scale(t1),
        line.origin + line.direction.scale(t2),
    )
    return points
