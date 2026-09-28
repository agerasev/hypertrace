//! Small DOM bridge; Wgame owns the canvas, input and browser event loop.
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/web/controls.js")]
extern "C" {
    pub fn supported() -> bool;
    pub fn add_example(id: &str, title: &str, description: &str, group: &str);
    pub fn scene_name() -> String;
    pub fn resolution() -> u32;
    pub fn take_reset() -> bool;
    pub fn paused() -> bool;
    pub fn toggle_pause();
    pub fn set_status(message: &str, error: bool);
    pub fn set_stats(width: u32, height: u32, samples: f64);
}
