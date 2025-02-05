use chrono::NaiveDateTime;
use crate::systems::commits::commit::Commit;
use crate::systems::init::get_current_branch;
use crate::systems::commits::functions::load_all_commits;

/// Log all commits
pub fn log() -> std::io::Result<()> {
    let branch = get_current_branch()?;  // Get the current branch
    let commits = load_all_commits()?;   // Load all commits

    // Filter commits for the current branch and collect into a vector
    let commits: Vec<&Commit> = commits.values()
        .filter(|commit| commit.branch == branch)
        .collect();

    println!("\tOn branch: {}\n", branch);

    for commit in &commits {
        let datetime = NaiveDateTime::from_timestamp(commit.timestamp as i64, 0);

        println!("commit {}", commit.id);
        println!("Author: {}", commit.author);
        println!("Date: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
        println!("\n{}\n", commit.message);
    }

    Ok(())
}
