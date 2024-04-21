use crate::rs_math_lib::intersection::intersect_line_sphere;
use crate::rs_math_lib::line::Line;
use crate::rs_math_lib::sphere::Sphere;
use crate::rs_math_lib::vector3::Vector3;

use super::camera::Camera;

pub fn color(camera: &Camera, sphere: &Sphere, x_coord: f64, y_coord: f64) -> [u8; 3] {
    let screen_ray = Line::new(Vector3::new(x_coord, y_coord, 0.0), camera.screen_direction);
    let intersections = intersect_line_sphere(&screen_ray, sphere);

    if intersections.is_none() {
        return [100, 100, 100]; // Gray background
    }

    // Take the point closest to the camera plane
    let (_, intersection_near) = intersections.unwrap();

    // Find color by using normal vector
    let normal = sphere.normal_at(intersection_near);
    let light_ray = (camera.light_source - intersection_near).normalize();

    let red_color = (normal.dot(&light_ray) * 255.0).max(30.0);
    [red_color as u8, 0, 0]
}
