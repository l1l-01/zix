use raylib::prelude::Color;

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub active: bool,
    pub color: Color,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub mode: String,
    pub color: Color,
}
