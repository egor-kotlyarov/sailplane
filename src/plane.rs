const M: f64 = 6.0;    // Масса планера
const S: f64 = 2.0;    // Площадь крыла
const C_L: f64 = 0.3;  // Подъёмная сила(коэф.)
const C_D: f64 = 0.05; // Сопротивление(коэф.)
const R: f64 = 1.225;  // Плотность воздуха
const G: f64 = 9.81;   // Ускорение свободного падения

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64
}
impl Plane {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64) -> Self {
        Self {x, y, vx, vy}
    }
    pub fn acceleration(&self) -> (f64, f64) {    
        let v: f64 = self.speed(); // Скорость

        let ux: f64 = self.vx / v;
        let uy: f64 = self.vy / v; // Направления скорости
        
        let q: f64 = (R * v * v * S) / 2.0;

        let drag: f64 = q * C_D;
        let lift: f64 = q * C_L;

        let fx: f64 = -drag * ux - lift * uy;
        let fy: f64 =  -drag * uy + lift * ux - M * G;

        (fx / M, fy / M)        
    }
   pub  fn speed(&self) -> f64 {
        (self.vx * self.vx + self.vy * self.vy).sqrt()
    }
   pub  fn step(&mut self, dt: f64) {
        let (ax, ay) = self.acceleration();

        self.vx += ax * dt;
        self.vy += ay * dt;

        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }
}
