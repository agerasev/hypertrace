//! Optional gallery viewer; individual scenes also have standalone binaries.
#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_examples::viewer::run_gallery(hypertrace_examples::EXAMPLES).await
}
