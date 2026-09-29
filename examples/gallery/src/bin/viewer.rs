//! Optional gallery viewer; individual scenes also have standalone binaries.
#[wgame::app]
async fn main() -> wgame::Result<()> {
    hypertrace_gallery::viewer::run_gallery().await
}
