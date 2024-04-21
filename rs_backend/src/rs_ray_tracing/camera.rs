use pyo3::prelude::*;

use crate::rs_math_lib::sphere::{PySphere, Sphere};
use crate::rs_math_lib::vector3::{PyVector3, Vector3};

use super::color::color;

pub struct Camera {
    pub light_source: Vector3,
    pub screen_center: Vector3,
    pub screen_direction: Vector3,
    pub screen_width_pixels: usize,
    pub screen_height_pixels: usize,
    pub screen_width: f64,
    pub screen_height: f64,
    pub pixel_width: f64,
    pub pixel_height: f64,
}

impl Camera {
    pub fn new(
        light_source: Vector3,
        screen_center: Vector3,
        screen_direction: Vector3,
        screen_width_pixels: usize,
        screen_height_pixels: usize,
        screen_width: f64,
        screen_height: f64,
    ) -> Camera {
        let pixel_width = screen_width / screen_width_pixels as f64;
        let pixel_height = screen_height / screen_height_pixels as f64;
        Camera {
            light_source,
            screen_center,
            screen_direction,
            screen_width_pixels,
            screen_height_pixels,
            screen_width,
            screen_height,
            pixel_width,
            pixel_height,
        }
    }

    pub fn get_pixel_color(&self, sphere: &Sphere, x_pixel: usize, y_pixel: usize) -> [u8; 3] {
        let pixel_x_coord = self.screen_center.x
            + self.pixel_width * (x_pixel as f64 - self.screen_width_pixels as f64 / 2.0);
        let pixel_y_coord = self.screen_center.y
            + self.pixel_height * (y_pixel as f64 - self.screen_height_pixels as f64 / 2.0);
        color(self, sphere, pixel_x_coord, pixel_y_coord)
    }

    pub fn fill_screen(&self, sphere: &Sphere) -> Vec<[u8; 3]> {
        let mut rgbs = Vec::with_capacity(self.screen_width_pixels * self.screen_height_pixels);
        for x in 0..self.screen_width_pixels {
            for y in 0..self.screen_height_pixels {
                rgbs.push(self.get_pixel_color(sphere, x, y));
            }
        }
        rgbs
    }
}

//----------------------------------------------------------------
// Python Bindings

#[pyclass(name = "Camera", frozen)]
pub struct PyCamera(pub(crate) Camera);

#[pymethods]
impl PyCamera {
    #[new]
    pub fn new(
        light_source: PyVector3,
        screen_center: PyVector3,
        screen_direction: PyVector3,
        screen_width_pixels: usize,
        screen_height_pixels: usize,
        screen_width: f64,
        screen_height: f64,
    ) -> Self {
        PyCamera(Camera::new(
            light_source.0,
            screen_center.0,
            screen_direction.0,
            screen_width_pixels,
            screen_height_pixels,
            screen_width,
            screen_height,
        ))
    }

    pub fn get_pixel_color(&self, sphere: &PySphere, x_pixel: usize, y_pixel: usize) -> [u8; 3] {
        self.0.get_pixel_color(&sphere.0, x_pixel, y_pixel)
    }

    pub fn fill_screen(&self, sphere: &PySphere) -> Vec<[u8; 3]> {
        self.0.fill_screen(&sphere.0)
    }

    // Accessor methods

    #[getter]
    pub fn light_source(&self) -> PyVector3 {
        PyVector3(self.0.light_source)
    }

    #[getter]
    pub fn screen_center(&self) -> PyVector3 {
        PyVector3(self.0.screen_center)
    }

    #[getter]
    pub fn screen_direction(&self) -> PyVector3 {
        PyVector3(self.0.screen_direction)
    }

    #[getter]
    pub fn screen_width_pixels(&self) -> usize {
        self.0.screen_width_pixels
    }

    #[getter]
    pub fn screen_height_pixels(&self) -> usize {
        self.0.screen_height_pixels
    }

    #[getter]
    pub fn screen_width(&self) -> f64 {
        self.0.screen_width
    }

    #[getter]
    pub fn screen_height(&self) -> f64 {
        self.0.screen_height
    }
}
