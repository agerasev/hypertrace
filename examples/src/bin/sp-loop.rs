//! Run the spherical long route example independently.
use objects::Scene as _;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run(hypertrace_examples::Example::new(
        "sp-loop",
        "Spherical long route",
        || hypertrace_examples::recurrence::scene::<1>(false).definition(),
    ))
    .await
}
