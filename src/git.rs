use std::process::Command;

pub fn list_local_branches() -> Vec<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--list")
        .output()
        .expect("Failed to execute git command");

    let branches: Vec<String> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| line.replace('*', "").trim().to_string())
        .collect();

    branches
}

pub fn list_remote_branches() -> Vec<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--list")
        .arg("--remotes")
        .output()
        .expect("Failed to execute git command");

    let branches: Vec<String> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| line.trim().chars().skip_while(|c| *c != '/').skip(1).collect())
        .collect();

    branches
}

pub fn switch_to_branch(branch: &str) {
    Command::new("git")
        .arg("switch")
        .arg(branch)
        .output()
        .expect("Failed to execute git command");
}
