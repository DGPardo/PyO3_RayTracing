use pyo3::prelude::*;

use crate::rs_math_lib::constants::VSMALL;
use crate::rs_math_lib::line::{Line, PyLine};
use crate::rs_math_lib::sphere::{PySphere, Sphere};
use crate::rs_math_lib::vector3::{PyVector3, Vector3};

/// Calculate the intersection of a line and a sphere.
/// There are 3 possible outcomes:
/// 1. The line intersects the sphere at 2 points.
/// 2. The line intersects the sphere at 1 point.
/// 3. The line does not intersect the sphere.
pub fn intersect_line_sphere(line: &Line, sphere: &Sphere) -> Option<(Vector3, Vector3)> {
    let l = line.origin - sphere.center;
    let a = line.direction.dot(&line.direction);
    let b = 2.0 * line.direction.dot(&l);
    let c = l.dot(&l) - sphere.radius * sphere.radius;
    let d = b * b - 4.0 * a * c;

    if d < VSMALL {
        return None;
    }

    let t1 = (-b - d.sqrt()) / (2.0 * a).max(VSMALL);
    let t2 = (-b + d.sqrt()) / (2.0 * a).max(VSMALL);

    let points = (
        line.origin + line.direction.scale(t1),
        line.origin + line.direction.scale(t2),
    );

    Some(points)
}

// ----------------------------------------------------------------------
// Python Bindings

#[pyfunction(name = "intersect_line_sphere")]
pub fn py_intersect_line_sphere(
    line: &PyLine,
    sphere: &PySphere,
) -> PyResult<Option<(PyVector3, PyVector3)>> {
    let points = intersect_line_sphere(&line.0, &sphere.0);
    match points {
        Some((p1, p2)) => Ok(Some((PyVector3(p1), PyVector3(p2)))),
        None => Ok(None),
    }
}
