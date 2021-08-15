extern crate skim;

use skim::prelude::{Skim, SkimOptionsBuilder};
use structopt::StructOpt;

mod config;
mod exec;
mod item;
mod source;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "e", long = "exec")]
    /// Execute the command
    exec: bool,
}

pub fn main() {
    let cfg = config::read();
    let opt = Options::from_args();

    let options = SkimOptionsBuilder::default()
        .margin(Some("1,2,1,2"))
        .build()
        .unwrap();

    let rx_item = source::get_items();

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| {
            if out.is_abort {
                Vec::new()
            } else {
                out.selected_items
            }
        })
        .unwrap_or_else(|| Vec::new());

    for selected_item in selected_items.iter() {
        let launcher_item = selected_item.as_any().downcast_ref::<item::Item>().unwrap();
        let cmd = launcher_item.cmd.to_string();

        if opt.exec {
            exec::exec(launcher_item, &cfg);
        } else {
            print!("{}", &cmd);
        }
    }
}
