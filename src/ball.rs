extern crate sdl2;
extern crate rand;

// SDL2
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::pixels;

// SDL2 gfx
use sdl2::gfx::primitives::DrawRenderer;

// Rand
use self::rand::Rng;

// Game
use drawable::Drawable;

const RADIUS: i32 = 5;
const INIT_SPEED: f64 = 2.0;
const STEP_UP: f64 = 0.5;
const MAX: f64 = 6.0;

// Ball state
pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub speed: (f64, f64),
    pub projected_x: i32,
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
            projected_x: 0,
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

    /// Performs some sort of dodgy raycast (not raycast) thing to the computer's side
    pub fn raycast(&mut self) {
        // Point at the head of the cast for the ball
        let mut point = (self.x, self.y);
        let mut mock_speed = self.speed;
        while point.1 - self.radius > 0 {
            // Check if the cast needs to deflect off the wall
            if point.0 < 0 + self.radius {
                point.0 = 0 + self.radius;
                mock_speed.0 *= -1.0;
            } else if point.0 > self.arena_dimensions.0 as i32 - self.radius {
                point.0 = self.arena_dimensions.0 as i32 - self.radius;
                mock_speed.0 *= -1.0;
            }
            point.0 += mock_speed.0 as i32;
            point.1 += mock_speed.1 as i32;
        }
        self.projected_x = point.0;
    }

    pub fn reverse(&mut self) {
        if self.speed.1 < 0.0 {
            if self.speed.1 < -MAX {
                self.speed.1 -= STEP_UP;
            }
            self.speed.1 *= -1.0
        } else if self.speed.1 > 0.0 {
            if self.speed.1 < MAX {
                self.speed.1 += STEP_UP;
            }
            self.speed.1 *= -1.0;
        }
    }
}

impl Drawable for Ball {
    fn update(&mut self) {
        self.x += self.speed.0 as i32;
        self.y += self.speed.1 as i32;

        // Check and handle bounds (left and right)
        if self.x < 0 + self.radius {
            self.x = 0 + self.radius;
            self.speed.0 *= -1.0;
        } else if self.x > self.arena_dimensions.0 as i32 - self.radius {
            self.x = self.arena_dimensions.0 as i32 - self.radius;
            self.speed.0 *= -1.0;
        }

        // Check and handle bounds (top and bottom)
        if self.y < 0 + self.radius {
            self.y = 0 + self.radius;
            self.speed.1 *= -1.0;
        } else if self.y > self.arena_dimensions.1 as i32 - self.radius {
            self.y = self.arena_dimensions.1 as i32 - self.radius;
            self.speed.1 *= -1.0;
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        let color = pixels::Color::RGB(255, 255, 255);
        // Draw the ball
        canvas
            .filled_circle(self.x as i16, self.y as i16, self.radius as i16, color)
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
