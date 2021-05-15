use clap::ArgMatches;

pub fn exec(arg_matches: &ArgMatches) {
    match arg_matches.value_of("file") {
        Some(file_name) => {
            println!("Passed file name is {}", file_name);
        }
        _ => {}
    }
}
