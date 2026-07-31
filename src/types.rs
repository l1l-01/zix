use raylib::prelude::Color;

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub active: bool,
    pub color: Color,
}

pub struct Player {
    pub width: i32,
    pub height: i32,
    pub active: bool,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub mode: String,
    pub color: Color,
}

pub struct Enemy {
    pub width: i32,
    pub height: i32,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub color: Color,
    pub active: bool,
    pub mode: String,
}
