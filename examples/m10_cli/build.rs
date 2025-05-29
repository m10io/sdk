use vergen::{BuildBuilder, Emitter, RustcBuilder};
use vergen_git2::Git2Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Build info
    let build = BuildBuilder::default().build_timestamp(true).build()?;

    // 2) Git info
    let git = Git2Builder::default()
        .branch(true)
        .commit_timestamp(true)
        .sha(true)
        .build()?;

    // 3) Rustc info
    let rustc = RustcBuilder::all_rustc()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&git)?
        .add_instructions(&rustc)?
        .emit()?;

    Ok(())
}
