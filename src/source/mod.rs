use crate::item;
use is_executable::IsExecutable;
use std::env;
use std::fs;
use std::path::Path;

use freedesktop_desktop_entry as desktop_entry;
use skim::prelude::{unbounded, Arc, SkimItemReceiver, SkimItemSender};

pub fn get_desktop_items() -> SkimItemReceiver {
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    for (_, entry) in desktop_entry::Iter::new(desktop_entry::default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&entry) {
            let entry = desktop_entry::decode(entry, &bytes);
            let target = if entry.terminal() {
                item::Target::TERMINAL
            } else {
                item::Target::DESKTOP
            };

            let _ = tx_item.send(Arc::new(item::Item::new(
                entry.name(Some("")).unwrap().to_string(),
                target,
                entry.exec().unwrap().to_string(),
                entry.comment(Some("")).unwrap_or("").to_string(),
            )));

            let actions = entry.actions().unwrap_or("").to_string();
            for action in actions.split(';').filter(|x| x.len() > 0) {
                let _ = tx_item.send(Arc::new(item::Item::new(
                    format!(
                        "{} ({})",
                        entry.name(Some("")).unwrap().to_string(),
                        entry.action_name(action, Some("")).unwrap().to_string()
                    ),
                    if entry.terminal() {
                        item::Target::TERMINAL
                    } else {
                        item::Target::DESKTOP
                    },
                    entry.action_exec(action).unwrap().to_string(),
                    "".to_string(),
                )));
            }
        }
    }

    let key = "PATH";
    let value: String = env::var(key).unwrap_or("".to_string());
    let dirs_path = value.split(':');

    for dir_path_str in dirs_path {
        let dir_path = Path::new(dir_path_str);
        for entry in fs::read_dir(dir_path).expect("Unable to read dir") {
            let entry = entry.expect("Unable to get entry");
            let path = entry.path();
            if path.is_executable() {
                let _ = tx_item.send(Arc::new(item::Item::new(
                    entry.file_name().to_str().unwrap().to_string(),
                    item::Target::TERMINAL,
                    path.to_str().unwrap().to_string(),
                    "".to_string(),
                )));
            }
        }
    }

    drop(tx_item);

    rx_item
}
