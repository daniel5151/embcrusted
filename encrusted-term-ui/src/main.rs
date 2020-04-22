#![deny(rust_2018_idioms)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use clap::{App, Arg};
use encrusted::{Options, Zmachine, UI};

mod ui_terminal;

use ui_terminal::TerminalUI;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("encrusted")
        .version(VERSION)
        .about("A zmachine interpreter")
        .arg(
            Arg::with_name("FILE")
                .help("Sets the story file to run")
                .required(true),
        )
        .get_matches();

    let path = Path::new(matches.value_of("FILE").unwrap());

    if !path.is_file() {
        println!(
            "\nCouldn't find game file: \n   {}\n",
            path.to_string_lossy()
        );
        process::exit(1);
    }

    let mut data = Vec::new();
    let mut file = File::open(path).expect("Error opening file");
    file.read_to_end(&mut data).expect("Error reading file");

    let ui = TerminalUI::new();

    let mut opts = Options::default();

    let rand32 = || rand::random();
    opts.rand_seed = [rand32(), rand32(), rand32(), rand32()];

    let mut zvm = Zmachine::new(data, ui, opts).expect("could not construct z-machine");

    zvm.run();
}
