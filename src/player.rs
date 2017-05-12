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
use paddle::Paddle;

const MOVESPEED: f64 = 5.0;
const INITIAL_HEALTH: u8 = 3;
const WIDTH: i32 = 50;
const HEIGHT: i32 = 10;

// Player state
pub struct Player {
    pub x: i32,
    pub y: i32,
    w: i32,
    h: i32,
    speed: f64,
    health: u8,
}

// Methods on the Player
impl Player {
    // Constructor for the Player
    pub fn new(canvas: &Canvas<Window>) -> Player {
        let window_size = canvas.window().size();
        // Return the player here
        Player {
            x: (window_size.0 as i32 - WIDTH) / 2,
            y: window_size.1 as i32 - HEIGHT,
            w: WIDTH,
            h: HEIGHT,
            speed: 0.0,
            health: INITIAL_HEALTH,
        }
    }
}

impl Paddle for Player {
    // Provide a method to reset the player if game restarted 
    fn reset(&mut self) {
        // TODO
    }

    fn move_left(&mut self, status: bool) {
        if status {
            self.speed = -MOVESPEED;
        } else {
            if self.speed < 0.0 {
                self.speed = 0.0;
            }

        }
    }

    fn move_right(&mut self, status: bool) {
        if status {
            self.speed = MOVESPEED;
        } else {
            if self.speed > 0.0 {
                self.speed = 0.0;
            }
        }
    }

    /// Call to lower the health of a player
    fn drop_health(&mut self) {
        self.health -= 1;
    }

    fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn touch(&mut self, b: &Ball) {
        // TODO
    }

    fn return_to_bounds(&mut self, arena_dimensions: (u32, u32)) {
        if self.x < 0 {
            self.x = 0;
            self.speed = 0.0;
        } else if self.x > arena_dimensions.0 as i32 - self.w {
            self.x = arena_dimensions.0 as i32 - self.w;
            self.speed = 0.0;
        }
    }
}

impl Drawable for Player {
    fn update(&mut self) {
        let new_x = self.x + self.speed as i32;

        self.x = new_x;        
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
            &Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                self.move_left(true);
            }
            &Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.move_right(true);
            }
            &Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                self.move_right(true);
            }
            _ => {} // Discard other keycodes for now
        }
    }

    fn on_key_up(&mut self, event: &Event) {
        match event {
            &Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                self.move_left(false);
            }
            &Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                self.move_left(false);
            }
            &Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                self.move_right(false);
            }
            &Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                self.move_right(false);
            }
            _ => {} // Discard other keycodes for now
        }
    }
}