use clap::ArgMatches;

pub fn exec(arg_matches: &ArgMatches) {
    match arg_matches.value_of("file") {
        Some(plugin_name) => {
            println!("Uninstalling plugin name is {}", plugin_name);
        }
        _ => {}
    }
}
