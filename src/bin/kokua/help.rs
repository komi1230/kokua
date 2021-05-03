pub fn help() {
    let description = "Yet Another Data Loader";

    let usage = "\
USAGE:
    launcher [FLAGS] <SUBCOMMAND>";

    let flags = "\
FLAGS:
    -V, --version    Print version info and exit
    -h, --help       Prints help information";

    let subcommands = "\
SUBCOMMAND:
    run         Run a bulk load transaction
    preview     Dry-run the bulk load without output and show preview
    install     Install plugin
    uninstall   Uninstall plugin
    new         Generate new plugin template";

    let help_subcommand = "\
    Use `<command> --help` to see description of the commands.";

    println!(
        "{description}\n\n{usage}\n\n{flags}\n\n{subcommands}\n\n{help_subcommand}",
        description = description,
        usage = usage,
        flags = flags,
        subcommands = subcommands,
        help_subcommand = help_subcommand,
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn hoge() {
        assert!(true);
    }
}
