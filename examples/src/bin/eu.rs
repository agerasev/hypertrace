//! Run the euclidean glass example independently.
use objects::Scene as _;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run(hypertrace_examples::Example::new(
        "eu",
        "Euclidean glass",
        || hypertrace_examples::eu::scene::<4>().definition(),
    ))
    .await
}
