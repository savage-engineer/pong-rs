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

const MOVESPEED: f64 = 2.0;
const INITIAL_HEALTH: u8 = 3;
const WIDTH: i32 = 50;
const HEIGHT: i32 = 10;

// Player state
pub struct Computer {
    pub x: i32,
    pub y: i32,
    w: i32,
    h: i32,
    target_x: i32,
    targeting: bool, 
    speed: f64,
    health: u8,
}

// Methods on the Computer 
impl Computer {
    // Constructor for the Computer 
    pub fn new(canvas: &Canvas<Window>) -> Computer {
        let window_size = canvas.window().size();
        // Return the Computer here
        Computer {
            x: (window_size.0 as i32 - WIDTH) / 2,
            y: 0,
            w: WIDTH,
            h: HEIGHT,
            target_x: 0,
            targeting: false,
            speed: 0.0,
            health: INITIAL_HEALTH,
        }
    }
}

impl Paddle for Computer {
    // Provide a method to reset the player if game restarted 
    fn reset(&mut self, centre: u32) {
        self.x = centre as i32 - (self.w/2);
        self.speed = 0.0;
        self.health = INITIAL_HEALTH;
    }

    fn move_left(&mut self, status: bool) {
        self.speed = -MOVESPEED;
    }

    fn move_right(&mut self, status: bool) {
        self.speed = MOVESPEED;
    }

    /// Call to lower the health of a player
    fn drop_health(&mut self) {
        self.health -= 1;
    }

    fn is_dead(&self) -> bool {
        self.health == 0
    }

    fn touch(&mut self, b: &mut Ball) {
        // First update internal note of position
        self.target_x = b.x;
        if b.speed.1 > 0.0 {
            self.targeting = false;
        } else {
            self.targeting = true;
        }
        // Check if hit
        if b.y - b.radius < self.y + self.h &&
           b.x < self.x + self.w &&
           b.x > self.x {
            b.y = self.y + b.radius + self.h;
            b.reverse();
        }

    }

    fn return_to_bounds(&mut self, arena_dimensions: (u32, u32)) {
        if self.x < 0 {
            
        } else if self.x > arena_dimensions.0 as i32 {

        }
    }
}

impl Drawable for Computer {
    fn update(&mut self) {
        self.x += self.speed as i32;
        // After movement reevaluate position and move appropriately
        if self.targeting {
            if self.target_x > self.x + (self.w/2) {
                self.move_right(false);
            } else if self.target_x < self.x + (self.w/2) {
                self.move_left(false);
            } 
        }
        
        if self.target_x < self.x + 5 * (self.w/9) &&
                  self.target_x > self.x + 4 * (self.w/9) {
            self.speed = 0.0;
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        // Set computer color to red 
        let color = pixels::Color::RGB(255, 0, 0); 
        // Draw computer 
        canvas.box_(self.x as i16,
                           self.y as i16,
                           (self.x + self.w) as i16,
                           (self.y + self.h) as i16,
                           color)
                           .expect("Computer should have rendered");
    }

    fn on_key_down(&mut self, event: &Event) {
        match event {
            _ => {}
        }
    }

    fn on_key_up(&mut self, event:&Event) {
        match event {
            _ => {}
        }
    }
}