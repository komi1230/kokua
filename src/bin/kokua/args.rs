use super::commands;
use clap::{App, AppSettings, Arg, ArgMatches};

pub fn cli() -> App<'static> {
    let subcmd_run = App::new("run").about("Run a bulk load transaction").arg(
        Arg::new("file")
            .value_name("FILE")
            .about("YAML or liquid file for config of data transaction")
            .takes_value(true),
    );

    let subcmd_install = App::new("install").about("Install a new plugin").arg(
        Arg::new("plugin")
            .value_name("PLUGIN")
            .about("Plugin you want to use")
            .takes_value(true),
    );

    let subcmd_uninstall = App::new("uninstall")
        .about("Uninstall an existing plugin")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .about("YAML or liquid file for config of data transaction")
                .takes_value(true),
        );

    App::new("kokua")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .version("0.1.0")
        .help_template("{about-with-newline}\n{usage-heading}\n    {usage}\n\n{all-args}")
        .about("Yet Another Embulk written in Rust")
        .subcommand(subcmd_run)
        .subcommand(subcmd_install)
        .subcommand(subcmd_uninstall)
}
