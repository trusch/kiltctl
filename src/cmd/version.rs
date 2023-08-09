pub fn command() -> clap::Command {
    clap::Command::new("version").about("Show version")
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Semver: {}", std::env::var("VERGEN_BUILD_SEMVER")?);
    println!("Build time: {}", std::env::var("VERGEN_BUILD_TIMESTAMP")?);
    println!("Git Semver: {}", std::env::var("VERGEN_GIT_SEMVER")?);
    println!("Git SHA: {}", std::env::var("VERGEN_GIT_SHA")?);
    println!("Git Branch: {}", std::env::var("VERGEN_GIT_BRANCH")?);
    println!("Git Commit date: {}", std::env::var("VERGEN_GIT_COMMIT_TIMESTAMP")?);
    Ok(())
}
