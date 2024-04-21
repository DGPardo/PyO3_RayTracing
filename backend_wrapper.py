import os

MATH_BACKEND = os.environ.get("MATH_BACKEND", None)
RAYTRACING_BACKEND = os.environ.get("RAYTRACING_BACKEND", None)

if MATH_BACKEND == "python":
    from py_backend.py_math_lib.vector3 import Vector3
    from py_backend.py_math_lib.sphere import Sphere
    from py_backend.py_math_lib.line import Line
    from py_backend.py_math_lib.intersection import intersect_line_sphere
elif MATH_BACKEND == "rust":
    from rs_backend import Vector3
    from rs_backend import Sphere
    from rs_backend import Line
    from rs_backend import intersect_line_sphere
else:
    raise ImportError(f"Unknown math backend: {MATH_BACKEND}")

if RAYTRACING_BACKEND == "python":
    from py_backend.py_ray_tracing.camera import Camera
elif RAYTRACING_BACKEND == "rust":
    from rs_backend import Camera
else:
    raise ImportError(f"Unknown ray tracing backend: {RAYTRACING_BACKEND}")
