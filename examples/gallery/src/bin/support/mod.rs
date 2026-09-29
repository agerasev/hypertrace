//! Compile the shared generic Rust scene builders.

use hypertrace_gallery as examples;

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
