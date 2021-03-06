extern crate sdl2;
extern crate rand;

// SDL2
use sdl2::render::Canvas;
use sdl2::video::Window;
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
            game_ended: true,
            arena_dimensions: canvas.window().size(),
            player: player.clone(),
            computer: computer.clone(),
            ball: ball.clone(),
            entities: entities,
        }
    }

    fn reset(&mut self) {
        let ref mut player = self.player.borrow_mut();
        let ref mut computer = self.computer.borrow_mut();
        let ref mut ball = self.ball.borrow_mut();

        player.reset(self.arena_dimensions.0 / 2);
        computer.reset(self.arena_dimensions.0 / 2);
        ball.reset();
        self.game_ended = true;
    }
}

impl Drawable for Scene {
    fn update(&mut self) {
        if !self.paused && !self.game_ended {

            let mut reset = false;

            for entity in &self.entities {
                entity.borrow_mut().update();
            }
            // Reset bounds
            {
                self.player.borrow_mut().return_to_bounds(self.arena_dimensions);
                self.computer.borrow_mut().return_to_bounds(self.arena_dimensions);
            }
            // Check for collision between ball and paddles
            {
                let ref mut ball = self.ball.borrow_mut();
                self.player.borrow_mut().touch(ball);
                self.computer.borrow_mut().touch(ball);
            }
            // Check for loss 
            {
                let ref mut ball = self.ball.borrow_mut();
                // Some grim hardcode here :^)
                if ball.y - ball.radius == 0 {
                    reset = true;
                } else if ball.y + ball.radius == self.arena_dimensions.1 as i32 {
                    reset = true;
                }
            }

            if reset == true {
                self.reset();
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
            &Event::KeyDown { keycode: Some(Keycode::P), repeat: false, .. } => {
                self.paused = !self.paused;
            }
            &Event::KeyDown { keycode: Some(Keycode::Return), repeat: false, .. } => {
                if !self.paused {
                    match self.game_ended {
                        true => {
                            self.game_ended = false;
                            self.ball.borrow_mut().kick_off();
                        }
                        false => {
                            self.reset();
                        }
                    }
                }
            }
            _ => {}
        }

        // Propagate events to scene entities
        if !self.paused && !self.game_ended {
            for entity in &self.entities {
                entity.borrow_mut().on_key_down(event);
            }
        }
    }

    fn on_key_up(&mut self, event: &Event) {
        match event {
            _ => {}
        }

        if !self.paused && !self.game_ended {
            for entity in &self.entities {
                entity.borrow_mut().on_key_up(event);
            }
        }
    }
}
