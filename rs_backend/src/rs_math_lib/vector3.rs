use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::ops::Sub;

use pyo3::prelude::*;

use crate::rs_math_lib::constants::VSMALL;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn scale(&self, s: f64) -> Vector3 {
        Vector3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Vector3 {
        self.scale(1.0 / self.norm().max(VSMALL))
    }
}

// Operator Overloading

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Vector3({}, {}, {})", self.x, self.y, self.z)
    }
}

// ----------------------------------------------------------------------
// Python Bindings

#[derive(Copy, Clone)]
#[pyclass(name = "Vector3")]
pub struct PyVector3(pub Vector3);

#[pymethods]
impl PyVector3 {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        PyVector3(Vector3::new(x, y, z))
    }

    pub fn norm(&self) -> f64 {
        self.0.norm()
    }

    pub fn scale(&self, s: f64) -> PyVector3 {
        PyVector3(self.0.scale(s))
    }

    pub fn dot(&self, other: &PyVector3) -> f64 {
        self.0.dot(&other.0)
    }

    pub fn normalize(&self) -> PyVector3 {
        PyVector3(self.0.normalize())
    }

    // Accessors
    #[getter]
    pub fn x(&self) -> PyResult<f64> {
        Ok(self.0.x)
    }

    #[getter]
    pub fn y(&self) -> PyResult<f64> {
        Ok(self.0.y)
    }

    #[getter]
    pub fn z(&self) -> PyResult<f64> {
        Ok(self.0.z)
    }

    // Magic Methods (operator overloading)

    pub fn __add__(&self, other: &PyVector3) -> PyVector3 {
        PyVector3(self.0 + other.0)
    }

    pub fn __sub__(&self, other: &PyVector3) -> PyVector3 {
        PyVector3(self.0 - other.0)
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self.0)
    }
}
