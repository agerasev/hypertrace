//! Run the perspective: flat example independently.
use objects::Scene as _;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run(
        hypertrace_examples::Example::new("compare-eu", "Perspective: flat"),
        || hypertrace_examples::comparison::scene::<0, 1>(1.0)?.definition(),
    )
    .await
}
