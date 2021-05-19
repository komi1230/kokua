use std::path::{Path, PathBuf};

fn config_dir() -> PathBuf {
    let mut path = dirs::home_dir().expect("Environment varialbe $HOME should be set.");
    path.push(".config");
    path.push("kokua");
    path
}

pub fn exec() {
    let path_buf = config_dir();
    let path = path_buf.as_path();
    std::fs::create_dir_all(path).expect("Something wrong in creating config dir.");

    // Create plugins dir
    let mut path = path.to_path_buf();
    path.push("plugins");
    std::fs::create_dir(path);
}
