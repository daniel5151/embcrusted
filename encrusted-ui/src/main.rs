use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use encrusted_embedded::{Options, Ui, Zmachine};

mod counting_alloc;

#[global_allocator]
static ALLOCATOR: counting_alloc::CountingAllocator = counting_alloc::CountingAllocator::new();

// for embedded use-cases, you should probably just include the story file as a byte-array. e.g:
// const ZORK: &[u8] = include_bytes!("../games/zork.z3");

fn main() {
    let mut data = Vec::new();
    let mut file = File::open(
        std::env::args()
            .nth(1)
            .expect("must pass story file as argument"),
    )
    .expect("Error opening file");
    file.read_to_end(&mut data).expect("Error reading file");

    println!(
        "|| pre reset: {} {}",
        ALLOCATOR.get_current_usage(),
        ALLOCATOR.get_high_watermark(),
    );

    println!("resetting alloc counter");
    ALLOCATOR.reset_counts();

    println!(
        "|| post reset: {} {}",
        ALLOCATOR.get_current_usage(),
        ALLOCATOR.get_high_watermark(),
    );

    let opts = Options { rand_seed: 0x1337 };
    let mut zvm = Zmachine::new(&data, DumbUi::default(), opts);

    println!(
        "|| before exec: {} {}",
        ALLOCATOR.get_current_usage(),
        ALLOCATOR.get_high_watermark(),
    );

    while !zvm.step() {
        zvm.ui.fill_input_buf();
        zvm.ack_input();
    }

    println!(
        "|| post exec: {} {}",
        ALLOCATOR.get_current_usage(),
        ALLOCATOR.get_high_watermark(),
    );
}

#[derive(Default)]
pub struct DumbUi {
    buf: String,
}

impl DumbUi {
    fn fill_input_buf(&mut self) {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        self.buf = input.trim().to_string();
    }
}

impl Ui for DumbUi {
    fn print(&self, text: &str) {
        io::stdout().write_all(text.as_ref()).expect("io error");
        io::stdout().flush().unwrap();
    }

    fn print_object(&mut self, object: &str) {
        self.print(object);
    }

    fn set_status_bar(&self, left: &str, right: &str) {
        let _ = (left, right);
        // self.print(&format!("{}  -  {}", left, right));
    }

    fn get_input_buf(&mut self) -> &str {
        &self.buf
    }
}
