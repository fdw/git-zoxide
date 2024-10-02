use crate::git::{list_branches, switch_to_branch};
use std::env;

mod git;
mod matcher;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        println!("git-zoxide needs at least one hint for the branch name");
        return;
    }

    let branches = list_branches();
    let best_match =
        matcher::find_best_branch(&branches, &args.iter().map(|it| it.as_str()).collect());

    match best_match {
        Some(branch) => switch_to_branch(branch),
        None => println!("Couldn't find a good match. Not switching."),
    }
}
