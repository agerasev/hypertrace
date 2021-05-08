mod empty;
mod isotropic;

pub use empty::*;
pub use isotropic::*; 

#[derive(Clone, Copy, Debug)]
pub enum Motion {
    Position(f64),
    Velocity(f64),
}

pub trait Controller {
    fn move_forward(&mut self, motion: Motion);
    fn move_right(&mut self, motion: Motion);
    fn move_up(&mut self, motion: Motion);

    fn rotate_yaw(&mut self, motion: Motion);
    fn rotate_pitch(&mut self, motion: Motion);
    fn rotate_roll(&mut self, motion: Motion);

    fn zoom(&mut self, motion: Motion);
    fn zoom_alt(&mut self, motion: Motion);

    fn step(&mut self, dt: f64);
}
