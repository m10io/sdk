use std::env;

use vergen::{BuildBuilder, Emitter, RustcBuilder};
use vergen_git2::Git2Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = BuildBuilder::default().build_timestamp(true).build()?;
    let rustc = RustcBuilder::all_rustc()?;

    let mut emitter = Emitter::default();
    emitter.add_instructions(&build)?.add_instructions(&rustc)?;

    // When a .git directory is available (typical local dev), let vergen-git2
    // populate VERGEN_GIT_*. When unavailable (e.g. Docker builds where .git
    // isn't in the context), fall back to env-var values supplied by the
    // caller, or "unknown" placeholders.
    match Git2Builder::default()
        .branch(true)
        .commit_timestamp(true)
        .sha(true)
        .build()
    {
        Ok(git) => {
            emitter.add_instructions(&git)?;
        }
        Err(_) => {
            for var in [
                "VERGEN_GIT_SHA",
                "VERGEN_GIT_BRANCH",
                "VERGEN_GIT_COMMIT_TIMESTAMP",
            ] {
                let val = env::var(var).unwrap_or_else(|_| "unknown".to_string());
                println!("cargo:rustc-env={var}={val}");
            }
        }
    }

    emitter.emit()?;

    // Short SHA (7 chars) for display in version string.
    // Priority: VERGEN_GIT_SHA env override (set in CI workflow) → git command → "unknown".
    let sha = env::var("VERGEN_GIT_SHA")
        .ok()
        .or_else(|| {
            std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| "unknown".to_string());
    let short_sha: String = sha.chars().take(7).collect();
    println!("cargo:rustc-env=GIT_SHA_SHORT={short_sha}");

    Ok(())
}
