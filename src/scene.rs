extern crate sdl2;

// SDL2
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Standard Stuff
use std::cell::RefCell;
use std::rc::Rc;

// Our stuff
use player::Player;
use computer::Computer;
use ball::Ball;
use paddle::Paddle;
use drawable::Drawable;

pub struct Scene {
    // State
    paused: bool,
    game_ended: bool,
    arena_dimensions: (u32, u32),
    // Players
    player: Rc<RefCell<Player>>,
    computer: Rc<RefCell<Computer>>,
    // Game objects
    ball: Rc<RefCell<Ball>>,
    // Drawable objects
    entities: Vec<Rc<RefCell<Drawable>>>,
}

impl Scene {
    pub fn new(canvas: &Canvas<Window>) -> Scene {
        let player = Rc::new(RefCell::new(Player::new(&canvas)));
        let computer = Rc::new(RefCell::new(Computer::new(&canvas)));
        let ball = Rc::new(RefCell::new(Ball::new(&canvas)));

        let mut entities: Vec<Rc<RefCell<Drawable>>> = Vec::new();
        entities.push(player.clone());
        entities.push(computer.clone());
        entities.push(ball.clone());

        Scene {
            paused: false,
            game_ended: false,
            arena_dimensions: canvas.window().size(),
            player: player.clone(),
            computer: computer.clone(),
            ball: ball.clone(),
            entities: entities,
        }
    }
}

impl Drawable for Scene {
    fn update(&mut self) {
        if !self.paused {
            for entity in &self.entities {
                entity.borrow_mut().update();
                
            }
            // Reset bounds
            {
                let ref mut player = self.player.borrow_mut();
                let ref mut computer = self.computer.borrow_mut();
                player.return_to_bounds(self.arena_dimensions);
                computer.return_to_bounds(self.arena_dimensions);
            }
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        for entity in &self.entities {
            entity.borrow_mut().draw(canvas);
        }
    }

    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                self.paused = !self.paused;
            }
            _ => {}
        }

        // Propagate events to scene entities
        if !self.paused {
            for entity in &self.entities {
                entity.borrow_mut().on_key_down(event);
            }
        }
    }

    fn on_key_up(&mut self, event: &Event) {
        match event {
            _ => {}
        }

        if !self.paused {
            for entity in &self.entities {
                entity.borrow_mut().on_key_up(event);
            }
        }
    }
}