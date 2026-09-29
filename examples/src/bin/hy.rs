//! Run the hyperbolic tilings example independently.
use objects::Scene as _;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run(
        hypertrace_examples::Example::new("hy", "Hyperbolic tilings"),
        || hypertrace_examples::hy::scene::<3>().definition(),
    )
    .await
}
