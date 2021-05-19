use std::path::PathBuf;

fn config_dir() -> PathBuf {
    let mut path = dirs::home_dir().expect("Environment varialbe $HOME should be set.");
    path.push(".kokua");
    path
}

pub fn exec() {
    let path_buf = config_dir();
    let path = path_buf.as_path();
    std::fs::create_dir_all(path).expect("Something wrong in creating config dir.");

    // Create bin dir
    let mut path_bin = path.to_path_buf();
    path_bin.push("bin");
    std::fs::create_dir_all(path_bin).unwrap();

    // Create plugins dir
    let mut path_plugins = path.to_path_buf();
    path_plugins.push("plugins");
    std::fs::create_dir_all(path_plugins).unwrap();
}
