extern crate sdl2;
extern crate gl;

// Not really sure what the best practice is to do here lmao
pub mod scene;
pub mod player;
pub mod computer;
pub mod ball;
pub mod drawable;
pub mod paddle;

// SDL2
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Our stuff
use scene::Scene;
use drawable::Drawable;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 480;
const TITLE: &str = "👌🏻😂 pong-rs 👌🏻😂";

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

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut scene = Scene::new(&mut canvas);

    'running: loop {
        // Process events coming from the event_pump
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { .. } => {
                    scene.on_key_down(&event);
                },
                Event::KeyUp { .. } => {
                    scene.on_key_up(&event);
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        // Clear canvas
        canvas.clear();  

        // Update and draw the scene
        scene.update();
        scene.draw(&mut canvas);

        // Show updated canvas
        canvas.present();
    }
}
