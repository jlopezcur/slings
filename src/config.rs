extern crate xdg;

pub fn read() -> toml::Value {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("slings").unwrap();
    let config_path = xdg_dirs
        .place_config_file("config.toml")
        .expect("Cannot create config file");
    let config_text = std::fs::read_to_string(config_path).unwrap();
    config_text.parse::<toml::Value>().unwrap()
}
