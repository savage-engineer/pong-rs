extern crate sdl2;
extern crate gl;

pub mod player;
pub mod drawable;
pub mod ball;

// SDL2
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Standard Stuff
use std::cell::RefCell;
use std::rc::Rc;

// Our stuff
use drawable::Drawable;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 480;
const TITLE: &str = "First Game 👌🏻😂";

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut tick = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let player = Rc::new(RefCell::new(player::Player::new(&canvas)));

    'running: loop {
        // Process events coming from the event_pump
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Tick 
        {
            tick += 1;
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        player.borrow().draw(&mut canvas);
        canvas.present();
    }
}
