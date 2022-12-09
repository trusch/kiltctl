pub fn command() -> clap::Command {
    clap::Command::new("version").about("Show version")
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Semver: {}", env!("VERGEN_BUILD_SEMVER"));
    println!("Build time: {}", env!("VERGEN_BUILD_TIMESTAMP"));
    println!("Git Semver: {}", env!("VERGEN_GIT_SEMVER"));
    println!("Git SHA: {}", env!("VERGEN_GIT_SHA"));
    println!("Git Branch: {}", env!("VERGEN_GIT_BRANCH"));
    println!("Git Commit date: {}", env!("VERGEN_GIT_COMMIT_TIMESTAMP"));
    Ok(())
}
