//! Run the perspective: spherical example independently.
use objects::Scene as _;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run(
        hypertrace_examples::Example::new("compare-sp", "Perspective: spherical"),
        || hypertrace_examples::comparison::scene::<1, 1>(1.0)?.definition(),
    )
    .await
}
