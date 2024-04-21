use std::fmt;
use std::fmt::{Display, Formatter};

use pyo3::prelude::*;

use crate::rs_math_lib::vector3::{PyVector3, Vector3};

#[derive(Copy, Clone)]
pub struct Line {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Line {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Line { origin, direction }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Line({}, {})", self.origin, self.direction)
    }
}

// ----------------------------------------------------------------------
// Python Bindings

#[derive(Copy, Clone)]
#[pyclass(name = "Line", frozen)]
pub struct PyLine(pub Line);

#[pymethods]
impl PyLine {
    #[new]
    pub fn new(origin: &PyVector3, direction: &PyVector3) -> Self {
        PyLine(Line::new(origin.0, direction.0))
    }

    #[getter]
    pub fn center(&self) -> PyResult<PyVector3> {
        Ok(PyVector3(self.0.origin))
    }

    #[getter]
    pub fn direction(&self) -> PyResult<PyVector3> {
        Ok(PyVector3(self.0.direction))
    }
}
