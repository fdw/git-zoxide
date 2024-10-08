use crate::git::{list_local_branches, list_remote_branches, switch_to_branch};
use std::env;

mod git;
mod matcher;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        println!("git-zoxide needs at least one hint for the branch name");
        return;
    }

    let local_branches = list_local_branches();
    let best_local_match =
        matcher::find_best_branch(&local_branches, &args.iter().map(|it| it.as_str()).collect());

    if !best_local_match.is_none() {
        switch_to_branch(best_local_match.unwrap());
        return
    }

    let remote_branches = list_remote_branches();
    let best_remote_match = matcher::find_best_branch(&remote_branches, &args.iter().map(|it| it.as_str()).collect());

    match best_remote_match {
        Some(branch) => switch_to_branch(branch),
        None => println!("Couldn't find a good match. Not switching."),
    }
}
