#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};

mod commands;

fn main() {
    match cli().get_matches().subcommand() {
        Some(("run", subcmd_matches)) => {
            commands::run::exec(subcmd_matches);
        }
        Some(("install", subcmd_matches)) => {
            commands::install::exec(subcmd_matches);
        }
        Some(("uninstall", subcmd_matches)) => {
            commands::uninstall::exec(subcmd_matches);
        }
        _ => {
            println!("hello");
        }
    }
}

pub fn cli<'a>() -> App<'a> {
    let subcmd_run = App::new("run")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .about("Run a bulk load transaction")
        .help_template("{about-with-newline}\n{usage-heading}\n    {usage}\n\n{all-args}")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .about("YAML or liquid file for config of data transaction")
                .takes_value(true),
        );

    let subcmd_install = App::new("install")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .about("Install a new plugin")
        .help_template("{about-with-newline}\n{usage-heading}\n    {usage}\n\n{all-args}")
        .arg(
            Arg::new("plugin")
                .value_name("PLUGIN")
                .about("Plugin you want to use")
                .takes_value(true),
        );

    let subcmd_uninstall = App::new("uninstall")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .about("Uninstall an existing plugin")
        .help_template("{about-with-newline}\n{usage-heading}\n    {usage}\n\n{all-args}")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .about("YAML or liquid file for config of data transaction")
                .takes_value(true),
        );

    App::new(crate_name!())
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .help_template("{about-with-newline}\n{usage-heading}\n    {usage}\n\n{all-args}")
        .about(crate_description!())
        .subcommand(subcmd_run)
        .subcommand(subcmd_install)
        .subcommand(subcmd_uninstall)
}
