use crate::item;
use freedesktop_desktop_entry as desktop_entry;
use skim::prelude::{unbounded, Arc, SkimItemReceiver, SkimItemSender};
use std::fs;

pub fn get_items() -> SkimItemReceiver {
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

    drop(tx_item);

    rx_item
}
