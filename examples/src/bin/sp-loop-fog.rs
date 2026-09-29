//! Run the spherical long route with fog example independently.
use objects::Scene as _;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run(hypertrace_examples::Example::new(
        "sp-loop-fog",
        "Spherical long route with fog",
        || hypertrace_examples::recurrence::scene::<12>(true).definition(),
    ))
    .await
}
