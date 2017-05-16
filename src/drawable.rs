extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;

pub trait Drawable {
    fn update(&mut self);

    fn draw(&self, canvas: &mut Canvas<Window>);

    fn on_key_down(&mut self, event: &Event);

    fn on_key_up(&mut self, event: &Event);
}
