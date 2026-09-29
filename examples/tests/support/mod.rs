use hypertrace_examples as examples;
use hypertrace_renderer::{
    Scene,
    shader::{Background, ObjectNode},
};

pub fn example(name: &str) -> Scene {
    Scene::from_definition(&examples::find(name).unwrap().definition().unwrap()).unwrap()
}

pub fn background(color: [f32; 3]) -> Scene {
    let mut definition = examples::find("eu").unwrap().definition().unwrap();
    definition.object = ObjectNode::Vector(vec![]);
    definition.background = Background::Constant(color);
    Scene::from_definition(&definition).unwrap()
}
