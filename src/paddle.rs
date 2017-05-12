use ball::Ball;

pub trait Paddle {
    fn reset(&mut self);

    fn move_left(&mut self, status: bool);

    fn move_right(&mut self, status: bool);

    fn drop_health(&mut self);

    fn is_dead(&self) -> bool;
    
    fn touch(&mut self, b: &Ball);
}