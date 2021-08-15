pub fn read() -> toml::Value {
    let config_text = std::fs::read_to_string("/home/jlopez/dev/launcher/config.toml").unwrap();
    config_text.parse::<toml::Value>().unwrap()
}
