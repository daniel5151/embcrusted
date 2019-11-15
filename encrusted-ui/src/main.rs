use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::process;

use encrusted::{Options, Ui, Zmachine};

fn main() {
    let path = std::env::args().nth(1).expect("must specify file to run");
    let path = Path::new(&path);

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

    let version = data[0];

    if version == 0 || version > 8 {
        println!(
            "\n\
             \"{}\" has an unsupported game version: {}\n\
             Is this a valid game file?\n",
            path.to_string_lossy(),
            version
        );
        process::exit(1);
    }

    let opts = Options { rand_seed: 0x1337 };
    let mut zvm = Zmachine::new(data, DumbUi, opts);

    zvm.run();
}

pub struct DumbUi;

impl Ui for DumbUi {
    fn print(&self, text: &str) {
        io::stdout().write_all(text.as_ref()).expect("io error");
        io::stdout().flush().unwrap();
    }

    fn print_object(&mut self, object: &str) {
        self.print(object);
    }

    fn clear(&self) {}
    fn set_status_bar(&self, left: &str, right: &str) {
        let _ = (left, right);
        // self.print(&format!("{}  -  {}", left, right));
    }

    fn get_user_input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        input.trim().to_string()
    }

    fn reset(&self) {
        println!();
    }
}
