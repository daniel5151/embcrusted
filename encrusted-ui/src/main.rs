use std::io;
use std::io::Write;

use encrusted_embedded::{Options, Ui, Zmachine};

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

struct Counter;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static HIGH_WATERMARK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), SeqCst);
            let a = ALLOCATED.load(SeqCst);
            if a > HIGH_WATERMARK.load(SeqCst) {
                HIGH_WATERMARK.store(a, SeqCst)
            }
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), SeqCst);
    }
}

#[global_allocator]
static A: Counter = Counter;

const ZORK: &[u8] = include_bytes!("../../../../Downloads/ztools/zork1-r88-s840726.z3");

fn main() {
    println!(
        "pre reset: {} {}",
        ALLOCATED.load(SeqCst),
        HIGH_WATERMARK.load(SeqCst)
    );

    println!("resetting alloc counter");
    ALLOCATED.store(0, SeqCst);
    HIGH_WATERMARK.store(0, SeqCst);

    println!(
        "post reset: {} {}",
        ALLOCATED.load(SeqCst),
        HIGH_WATERMARK.load(SeqCst)
    );

    let opts = Options { rand_seed: 0x1337 };
    let mut zvm = Zmachine::new(&ZORK, DumbUi::default(), opts);

    println!(
        "before exec: {} {}",
        ALLOCATED.load(SeqCst),
        HIGH_WATERMARK.load(SeqCst)
    );

    while !zvm.step() {
        zvm.ui.fill_input_buf();
        zvm.ack_input();
    }

    println!(
        "post exec: {} {}",
        ALLOCATED.load(SeqCst),
        HIGH_WATERMARK.load(SeqCst)
    );
}

pub struct DumbUi {
    len: usize,
    buf: [u8; 64],
}

impl Default for DumbUi {
    fn default() -> DumbUi {
        DumbUi {
            len: 0,
            buf: [0; 64],
        }
    }
}

impl DumbUi {
    fn fill_input_buf(&mut self) {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        let s = input.trim().to_string();

        self.len = s.len();
        self.buf[..s.len()].copy_from_slice(s.as_ref());
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
        let raw = &self.buf[..self.len];
        unsafe { core::str::from_utf8_unchecked(raw) }
    }
}
