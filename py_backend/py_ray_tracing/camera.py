from dataclasses import dataclass
from functools import cached_property
from typing import List, Tuple

from backend_wrapper import Vector3, Sphere
from py_backend.py_ray_tracing.color import color


@dataclass
class Camera:
    light_source: Vector3  # position of the light source
    screen_center: Vector3
    screen_direction: Vector3
    screen_width_pixels: int
    screen_height_pixels: int
    screen_width: float
    screen_height: float

    @cached_property
    def pixel_width(self) -> float:
        return self.screen_width / self.screen_width_pixels

    @cached_property
    def pixel_height(self) -> float:
        return self.screen_height / self.screen_height_pixels

    def get_pixel_color(
        self, sphere: Sphere, x_pixel: int, y_pixel: int
    ) -> List[Tuple]:
        pixel_x_coord = self.screen_center.x + self.pixel_width * (
            x_pixel - self.screen_width_pixels / 2.0
        )
        pixel_y_coord = self.screen_center.y + self.pixel_height * (
            y_pixel - self.screen_height_pixels / 2.0
        )
        return color(self, sphere, pixel_x_coord, pixel_y_coord)

    def render(self, sphere: Sphere, pygame_window):
        for y in range(self.screen_height_pixels):
            for x in range(self.screen_width_pixels):
                rgb = self.get_pixel_color(sphere, x, y)
                pygame_window.set_at((x, y), rgb)

    def fill_screen(self, sphere: Sphere) -> List[Tuple]:
        rgbs = []
        for x in range(self.screen_width_pixels):
            for y in range(self.screen_height_pixels):
                rgbs.append(self.get_pixel_color(sphere, x, y))
        return rgbs
