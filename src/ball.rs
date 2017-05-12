extern crate sdl2;
extern crate rand;

// SDL2
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

// SDL2 gfx
use sdl2::gfx::primitives::DrawRenderer;

// Rand
use self::rand::Rng;

// Game
use paddle::Paddle;
use drawable::Drawable;

const RADIUS: i32 = 5;
const INIT_SPEED: f64 = 2.0;
const STEP_UP: f64 = 1.0;

// Ball state
pub struct Ball {
    pub x: i32,
    pub y: i32,
    radius: i32,
    speed: (f64, f64),
    arena_dimensions: (u32, u32),
}

impl Ball {
    pub fn new(canvas: &Canvas<Window>) -> Ball {
        let window_size = canvas.window().size();

        // Return a Ball
        Ball {
            x: (window_size.0 as i32 - 2 * RADIUS) / 2,
            y: (window_size.1 as i32 - 2 * RADIUS) / 2,
            radius: RADIUS,
            speed: (0.0, 0.0),
            arena_dimensions: window_size,
        }
    }

    pub fn reset(&mut self) {
        self.x = (self.arena_dimensions.0 as i32 - 2 * RADIUS) / 2; 
        self.y = (self.arena_dimensions.1 as i32 - 2 * RADIUS) / 2; 
        self.speed = (0.0, 0.0);
    }

    pub fn kick_off(&mut self) {
        let mut rng_dir = rand::thread_rng();
        let multipliers = rng_dir.gen::<(f64, f64)>();
        self.speed = (INIT_SPEED * multipliers.0, INIT_SPEED * multipliers.1 + 1.0);
    } 

    pub fn reverse(&mut self, paddle: &Paddle) {
        if self.speed.1 < 0.0 {
            self.speed.1 -= STEP_UP;
            self.speed.1 *= -1.0
        } else if self.speed.1 < 0.0 {
            self.speed.1 += STEP_UP;
            self.speed.1 *= -1.0;
        }
    }
}

impl Drawable for Ball {
    fn update(&mut self) {
        self.x += self.speed.0 as i32;
        self.y += self.speed.1 as i32;

        // Check and handle bounds (left and right)
        if self.x <= 0 {
            self.x = 0;
            self.speed.0 *= -1.0;
        } else if self.x >= self.arena_dimensions.0 as i32 - self.radius * 2 {
            self.x = self.arena_dimensions.0 as i32 - self.radius * 2;
            self.speed.0 *= -1.0;
        }

        // Check and handle bounds (top and bottom)
        if self.y <= 0 {
            self.y = 0;
            self.speed.1 *= -1.0;
        } else if self.y >= self.arena_dimensions.1 as i32 - self.radius * 2 {
            self.y = self.arena_dimensions.1 as i32 - self.radius * 2;
            self.speed.1 *= -1.0;
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        let color = pixels::Color::RGB(255, 255, 255);
        // Draw the ball
        canvas.filled_circle(self.x as i16,
                             self.y as i16,
                             self.radius as i16,
                             color)
                             .expect("Ball should have rendered"); 
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