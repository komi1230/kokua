use clap::ArgMatches;

pub fn exec(arg_matches: &ArgMatches) {
    match arg_matches.value_of("plugin") {
        Some(plugin_name) => {
            println!("Installing plugin is {}", plugin_name);
        }
        _ => {}
    }
}
