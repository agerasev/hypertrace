use ccgeom::Flat3;
use hypertrace_gallery as examples;
use hypertrace_renderer::{
    Scene,
    shader::{Background, Geometry, Result, SceneDefinition},
};

pub fn scene<G: Geometry>(factory: fn() -> Result<SceneDefinition<G>>) -> Scene<G> {
    Scene::from_definition(&factory().unwrap()).unwrap()
}

pub fn background(color: [f32; 3]) -> Scene<Flat3> {
    let mut definition = examples::factories::euclidean().unwrap();
    definition.objects.clear();
    definition.background = Background::constant(color);
    Scene::from_definition(&definition).unwrap()
}
