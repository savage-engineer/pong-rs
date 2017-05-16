use ball::Ball;

pub trait Paddle {
    fn reset(&mut self, centre: u32);

    fn move_left(&mut self, status: bool);

    fn move_right(&mut self, status: bool);

    fn drop_health(&mut self);

    fn is_dead(&self) -> bool;

    fn touch(&mut self, b: &mut Ball);

    fn return_to_bounds(&mut self, arena_dimensions: (u32, u32));
}
