use crate::rs_math_lib::intersection::py_intersect_line_sphere;
use crate::rs_math_lib::line::PyLine;
use crate::rs_math_lib::sphere::PySphere;
use crate::rs_math_lib::vector3::PyVector3;
use crate::rs_ray_tracing::camera::PyCamera;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn rs_backend(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register Classess
    m.add_class::<PyVector3>()?;
    m.add_class::<PySphere>()?;
    m.add_class::<PyLine>()?;
    m.add_class::<PyCamera>()?;

    // Register Functions
    m.add_function(wrap_pyfunction!(py_intersect_line_sphere, m)?)?;
    Ok(())
}
