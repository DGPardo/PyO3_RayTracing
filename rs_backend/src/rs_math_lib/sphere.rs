use std::fmt;
use std::fmt::{Display, Formatter};

use pyo3::prelude::*;

use crate::rs_math_lib::vector3::{PyVector3, Vector3};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub fn normal_at(&self, point: Vector3) -> Vector3 {
        (point - self.center).normalize()
    }
}

// Operator Overloading

impl Display for Sphere {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Sphere({}, {})", self.center, self.radius)
    }
}

// ----------------------------------------------------------------------
// Python Bindings

#[derive(Copy, Clone)]
#[pyclass(name = "Sphere")]
pub struct PySphere(pub Sphere);

#[pymethods]
impl PySphere {
    #[new]
    pub fn new(center: &PyVector3, radius: f64) -> Self {
        PySphere(Sphere::new(center.0, radius))
    }

    pub fn normal_at(&self, point: &PyVector3) -> PyVector3 {
        PyVector3(self.0.normal_at(point.0))
    }

    // Accessors
    #[getter]
    pub fn center(&self) -> PyResult<PyVector3> {
        Ok(PyVector3(self.0.center))
    }

    #[getter]
    pub fn radius(&self) -> PyResult<f64> {
        Ok(self.0.radius)
    }

    // Magic methods
    pub fn __repr__(&self) -> String {
        format!("{}", self.0)
    }
}
