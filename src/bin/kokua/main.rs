mod args;
mod commands;

fn main() {
    let matches = args::cli().get_matches();

    match matches.subcommand() {
        Some(("run", subcmd)) => (),
        _ => {
            args::cli().get_matches_from(vec!["kokua", "-h"]);
        }
    }
}
