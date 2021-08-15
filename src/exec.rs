use crate::item;
use execute::shell;
use nix::unistd::setsid;
use std::process::Stdio;

pub fn exec(item: &item::Item, cfg: &toml::Value) {
    match setsid() {
        Ok(_) => {}
        Err(_) => {}
    }
    match item.target {
        item::Target::DESKTOP => {
            shell(&item.cmd)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to start");
        }
        item::Target::TERMINAL => {
            let term_cmd = format!("{} -e {}", cfg.get("terminal").unwrap(), &item.cmd);
            shell(&term_cmd).spawn().unwrap();
        }
    }
}
