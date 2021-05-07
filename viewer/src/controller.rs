pub trait Controller {
    fn move_forward(&mut self, value: f64);
    fn move_right(&mut self, value: f64);
    fn move_up(&mut self, value: f64);

    fn rotate_yaw(&mut self, value: f64);
    fn rotate_pitch(&mut self, value: f64);
    fn rotate_roll(&mut self, value: f64);

    fn zoom(&mut self, value: f64);
    fn zoom_alt(&mut self, value: f64);
}

pub struct EmptyController;
impl Controller for EmptyController {
    fn move_forward(&mut self, _: f64) {}
    fn move_right(&mut self, _: f64) {}
    fn move_up(&mut self, _: f64) {}

    fn rotate_yaw(&mut self, _: f64) {}
    fn rotate_pitch(&mut self, _: f64) {}
    fn rotate_roll(&mut self, _: f64) {}

    fn zoom(&mut self, _: f64) {}
    fn zoom_alt(&mut self, _: f64) {}
}
