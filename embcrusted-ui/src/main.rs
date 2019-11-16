use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use embcrusted::{Options, Ui, Zmachine};

mod counting_alloc;

#[global_allocator]
static ALLOCATOR: counting_alloc::CountingAllocator = counting_alloc::CountingAllocator::new();

fn main() {
    let mut data = Vec::new();
    let mut file = File::open(
        std::env::args()
            .nth(1)
            .expect("must pass story file as argument"),
    )
    .expect("Error opening file");
    file.read_to_end(&mut data).expect("Error reading file");
    drop(file);

    println!(
        "|| (alloc, high water mark) at start of main: ({}, {})",
        ALLOCATOR.get_current_usage(),
        ALLOCATOR.get_high_watermark(),
    );

    let static_start = ((data[0x0E] as usize) << 8) + (data[0x0F] as usize);
    println!("|| game will require {} bytes of dyn mem", static_start);

    println!("|| ... resetting alloc counters prior to constructing interpreter");
    ALLOCATOR.reset_counts();

    let opts = Options { rand_seed: 0x1337 };
    let mut zvm = Zmachine::new(&data, DumbUi::default(), opts);

    let pre_exec_alloc = ALLOCATOR.get_current_usage();
    let pre_exec_high_watermark = ALLOCATOR.get_high_watermark();

    println!(
        "|| (alloc, high water mark) before gameplay: ({}, {})",
        pre_exec_alloc, pre_exec_high_watermark,
    );

    println!(
        "|| interpreter base memory usage => {} - {} = ({})",
        pre_exec_alloc,
        static_start,
        pre_exec_alloc - static_start
    );

    println!("|| resetting counters");
    ALLOCATOR.reset_counts();

    while !zvm.step() {
        zvm.ui.fill_input_buf();
        zvm.ack_input();
    }

    let current_usage = ALLOCATOR.get_current_usage();
    let high_watermark = ALLOCATOR.get_high_watermark();

    println!(
        "|| (alloc, high water mark) after gameplay: ({}, {})",
        current_usage, high_watermark,
    );

    println!(
        "|| final high water mark: {} + {} = ({})",
        pre_exec_alloc,
        high_watermark,
        high_watermark + pre_exec_alloc
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
