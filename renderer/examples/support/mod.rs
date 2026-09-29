//! Compile the shared generic Rust scene builders.

use hypertrace_renderer::{Result, Scene};

pub fn catalog() -> String {
    examples::EXAMPLES
        .iter()
        .map(|example| {
            format!(
                "  {:20} {}\n    {}",
                example.id, example.title, example.description
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn scene(name: &str) -> Result<Scene> {
    let example = examples::find(name).ok_or_else(|| {
        anyhow::anyhow!("unknown example {name:?}; choose one of:\n{}", catalog())
    })?;
    let definition = example.definition()?;
    Scene::from_definition(&definition)
}
