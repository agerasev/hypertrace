//! Run the spherical studio with fog example independently.
use objects::Scene as _;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run(hypertrace_examples::Example::new(
        "sp-fog",
        "Spherical studio with fog",
        || hypertrace_examples::sp::fog_scene::<12>().definition(),
    ))
    .await
}
