extern crate sdl2;

// Ball state
pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub r: i32,
    magnitude: f64,
    direction: f64
}