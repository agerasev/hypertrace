use crate::{Controller, Motion};

pub struct EmptyController;

impl Controller for EmptyController {
    fn move_forward(&mut self, _: Motion) {}
    fn move_right(&mut self, _: Motion) {}
    fn move_up(&mut self, _: Motion) {}

    fn rotate_yaw(&mut self, _: Motion) {}
    fn rotate_pitch(&mut self, _: Motion) {}
    fn rotate_roll(&mut self, _: Motion) {}

    fn zoom(&mut self, _: Motion) {}
    fn zoom_alt(&mut self, _: Motion) {}

    fn step(&mut self, _: f64) {}
}
