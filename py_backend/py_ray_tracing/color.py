from backend_wrapper import Vector3, Line, intersect_line_sphere


def color(
    camera, sphere, x_coord, y_coord
) -> (int, int, int):  # returns rgb colors in the range [0, 255]
    screen_ray = Line(Vector3(x_coord, y_coord, 0.0), camera.screen_direction)
    intersections = intersect_line_sphere(screen_ray, sphere)

    if intersections is None:
        return (100, 100, 100)  # Gray background

    # Take the point closest to the camera plane
    (intersection_far, intersection_near) = intersections

    # Find color by using normal vector
    normal = sphere.normal_at(intersection_near)
    light_ray = (camera.light_source - intersection_near).normalize()

    return (
        max(normal.dot(light_ray) * 255, 30),
        0.0,
        0.0,
    )
