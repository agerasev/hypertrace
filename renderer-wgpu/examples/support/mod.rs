//! Compile the shared generic Rust scene builders.

use hypertrace_wgpu::{Result, Scene};
use objects::Scene as _;

pub fn scene(name: &str) -> Result<Scene> {
    let definition = match name {
        "eu" => scenes::eu::scene::<4>().wgsl_scene()?,
        "hy" => scenes::hy::scene::<3>().wgsl_scene()?,
        "sp" => scenes::sp::scene::<6>().wgsl_scene()?,
        _ => anyhow::bail!("scene must be eu, hy, or sp"),
    };
    Scene::from_definition(&definition)
}
