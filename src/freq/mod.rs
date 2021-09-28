pub fn get_path() -> PathBuf {
    xdg::BaseDirectories::with_prefix("slings")
        .unwrap()
        .place_data_file("frequency.toml")
        .expect("Cannot create frecuency file")
}

pub fn read(path: PathBuf) -> toml::Value {
    std::fs::read_to_string(path)
        .or_else("".to_string())
        .parse::<toml::Value>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path() {
        let a = get_path();
        assert_eq!(a.to_string(), "".to_string());
    }

    #[test]
    fn test_get_pp() {
        assert_eq!(1, 2);
    }
}
