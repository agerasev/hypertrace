use crate::{Controller, Motion};
use ccgeom::{Geometry3, Map};

pub struct IsotropicController<G: Geometry3> {
    map: G::Map,
    zoom: f64,
    updated: bool,
}

impl<G: Geometry3> IsotropicController<G>
where
    G::Map: Copy,
{
    pub fn new(map: G::Map, zoom: f64) -> Self {
        Self {
            map,
            zoom,
            updated: true,
        }
    }
    pub fn update_map(&mut self, chain: G::Map) {
        self.map = self.map.chain(chain);
        self.updated = true;
    }
    pub fn map(&self) -> &G::Map {
        &self.map
    }
    pub fn zoom(&self) -> f64 {
        self.zoom
    }
    pub fn updated(&self) -> bool {
        self.updated
    }
}

impl<G: Geometry3> Default for IsotropicController<G>
where
    G::Map: Copy,
{
    fn default() -> Self {
        Self {
            map: G::Map::identity(),
            zoom: 1.0,
            updated: true,
        }
    }
}

const SPEED: f64 = 4.0;
const SENS: f64 = 1.0;

const DT: f64 = 0.04;

fn motion_to_value(m: Motion) -> f64 {
    match m {
        Motion::Position(x) => x,
        Motion::Velocity(x) => x * DT,
    }
}

impl<G: Geometry3> Controller for IsotropicController<G>
where
    G::Map: Copy,
{
    fn move_forward(&mut self, m: Motion) {
        self.update_map(G::shift_z(SPEED * motion_to_value(m)));
    }
    fn move_right(&mut self, m: Motion) {
        self.update_map(G::shift_x(SPEED * -motion_to_value(m)));
    }
    fn move_up(&mut self, m: Motion) {
        self.update_map(G::shift_y(SPEED * -motion_to_value(m)));
    }

    fn rotate_yaw(&mut self, m: Motion) {
        self.update_map(G::rotate_y(SENS * -motion_to_value(m)));
    }
    fn rotate_pitch(&mut self, m: Motion) {
        self.update_map(G::rotate_x(SENS * -motion_to_value(m)));
    }
    fn rotate_roll(&mut self, m: Motion) {
        self.update_map(G::rotate_z(SENS * -motion_to_value(m)));
    }

    fn zoom(&mut self, m: Motion) {
        let z = motion_to_value(m);
        self.zoom *= 1.2f64.powf(z);
        self.updated = true;
    }
    fn zoom_alt(&mut self, _m: Motion) {
        println!("zoom_alt is not implemented yet");
    }

    fn step(&mut self, _dt: f64) {
        self.updated = false;
    }
}
