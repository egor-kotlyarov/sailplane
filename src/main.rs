mod plane;
mod render;
use plane::Plane;
use macroquad::prelude::*;

#[macroquad::main("Glider")]
async fn main() {
    let mut p: Plane = Plane::new(0.0, 20.0, 12.0, 0.0);
    let dt: f64 = 0.01;

    let mut trail: Vec<(f64, f64)> = Vec::new();

    loop {
        if p.y > 0.0 {
            p.step(dt);
            p.step(dt);
            trail.push((p.x, p.y));
        }
        clear_background(WHITE);
        render::draw_ground();
        render::draw_trail(&trail);
        render::draw_plane(&p);

        next_frame().await;
    }
    
    let mut t: f64 = 0.0;

    while p.y > 0.0 {
        p.step(dt);
        t += dt;
    }
    println!("t = {:.2}, x = {:.2}, V = {:.2}", t, p.x, p.speed());
}
