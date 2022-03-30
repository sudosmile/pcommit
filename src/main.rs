use colored::*;

mod git;
mod input;
mod lib;
use crate::lib::*;

fn main() {
    // TODO: add a -h/--help
    println!(
        "commit types :\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
        "feat".red(),
        "fix".blue(),
        "docs".yellow(),
        "refactor".green(),
        "test".magenta(),
    );
    let msg = CommitMsg::new(input::get_type(), input::get_desc(), input::get_body());
    git::add();
    git::commit(msg);
    git::push();
}
