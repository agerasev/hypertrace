//! Run the spherical studio example independently.
use objects::Scene as _;

#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run(
        hypertrace_examples::Example::new("sp", "Spherical studio"),
        || hypertrace_examples::sp::scene::<6>().definition(),
    )
    .await
}
