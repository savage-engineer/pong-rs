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
use ball::Ball;
use drawable::Drawable;

const MOVESPEED: f64 = 2.0;
const INITIAL_HEALTH: u8 = 3;

// Player state
pub struct Player {
    pub x: i32,
    pub y: i32,
    w: i32,
    h: i32,
    speed: f64,
    health: u8
}

// Methods on the Player
impl Player {
    // Constructor for the Player
    pub fn new(canvas: &Canvas<Window>) -> Player {
        // Return the player here
        Player {
            x: 0,
            y: 0,
            w: 50,
            h: 10,
            speed: 0.0,
            health: INITIAL_HEALTH,
        }
    }

    // Provide a method to reset the player if game restarted 
    pub fn reset(&mut self) {
        // TODO
    }

    pub fn move_left(&mut self, status: bool) {
        // TODO
    }

    pub fn move_right(&mut self, status: bool) {
        // TODO
    }

    /// Call to lower the health of a player
    pub fn drop_health(&mut self) {
        self.health -= 1;
    }

    pub fn is_dead(&self) -> bool {
        return self.health == 0;
    }

    pub fn touch(&mut self, b: &Ball) {
        // TODO
    }
}

impl Drawable for Player {
    fn update(&mut self) {
        // TODO
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        // Set player color to white
        let color = pixels::Color::RGB(255, 255, 255); 
        // Draw player
        canvas.rectangle(self.x as i16,
                           self.y as i16,
                           (self.x + self.w) as i16,
                           (self.y + self.h) as i16,
                           color)
                           .expect("Player should have rendered");
    }

    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                self.move_left(true);
            }
            &Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.move_right(true);
            }
            _ => {} // Discard other keycodes for now
        }
    }

    fn on_key_up(&mut self, event:&Event) {
        match event {
            &Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                self.move_left(false);
            }
            &Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                self.move_right(false);
            }
            _ => {} // Discard other keycodes for now
        }
    }
}