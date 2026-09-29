//! Run the perspective: gentler hyperbolic example independently.
use objects::Scene as _;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run(
        hypertrace_examples::Example::new("compare-hy-flat", "Perspective: gentler hyperbolic"),
        || hypertrace_examples::comparison::scene::<-1, 1>(3.0)?.definition(),
    )
    .await
}
