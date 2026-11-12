use macroquad::prelude::*;
use crate::plane::Plane;

const SCALE: f32 = 8.0;
const LEFT: f32 = 20.0;
const GROUND: f32 = 50.0;

pub fn to_screen(x: f64, y: f64) -> (f32, f32) {
    let sx = LEFT + x as f32 * SCALE;
    let sy = screen_height() - GROUND - y as f32 * SCALE;
    (sx, sy)
}

pub fn draw_ground() {
    let (_, gy) = to_screen(0.0, 0.0);
    draw_line(0.0, gy, screen_width(), gy, 2.0, DARKGREEN);
}

pub fn draw_plane(p: &Plane) {
    let (sx, sy) = to_screen(p.x, p.y);
    draw_circle(sx, sy, 7.0, BLACK);
}

pub fn draw_trail(trail: &[(f64, f64)]) {
    for pair in trail.windows(2) {
        let (x1, y1) = to_screen(pair[0].0, pair[0].1);
        let (x2, y2) = to_screen(pair[1].0, pair[1].1);
        draw_line(x1, y1, x2, y2, 2.0, RED);
    }
}
