extern crate sdl2;

// SDL2
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

// SDL2 gfx
use sdl2::gfx::primitives::DrawRenderer;

// Game
use drawable::Drawable;

// Ball state
pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub r: i32,
    magnitude: f64,
    direction: f64
}

impl Ball {
    pub fn new(canvas: &Canvas<Window>) -> Ball {
        Ball {
            x: 0,
            y: 0,
            r: 5,
            magnitude: 0.0,
            direction: 0.0
        }
    }
}

impl Drawable for Ball {
    fn update(&mut self) {

    }

    fn draw(&self, canvas: &mut Canvas<Window>) {

    }

    fn on_key_down(&mut self, event: &Event) {
        match event {
            _ => {}
        }
    }

    fn on_key_up(&mut self, event: &Event) {
        match event {
            _ => {}
        }
    }
}