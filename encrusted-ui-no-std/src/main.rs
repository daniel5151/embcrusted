// some no_std boilerplate
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// use malloc/free from libc
mod malloc_alloc;
use malloc_alloc::MallocFreeAlloc;

#[global_allocator]
static ALLOCATOR: MallocFreeAlloc = MallocFreeAlloc::new();

/// custom print macro for no_std (redirects to printf)
macro_rules! print {
    ($($arg:tt)*) => {
        let string = alloc::fmt::format(format_args!($($arg)*));
        unsafe {
            libc::printf(format!("{}\0", string).as_ptr() as *const i8);
        }
    };
}

/// custom println macro for no_std (redirects to printf)
macro_rules! println {
    ($($arg:tt)*) => {
        let string = alloc::fmt::format(format_args!($($arg)*));
        unsafe {
            libc::printf(format!("{}\n\0", string).as_ptr() as *const i8);
        }
    };
}

////////////////////////////////////////////////////////////////////////////////

use encrusted_embedded::{Options, Ui, Zmachine};

const DATA: &[u8] = include_bytes!("../games/zork1.z3");

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!(
        "|| (alloc, high water mark) at start of main: ({}, {})",
        ALLOCATOR.get_current_usage(),
        ALLOCATOR.get_high_watermark(),
    );

    let static_start = ((DATA[0x0E] as usize) << 8) + (DATA[0x0F] as usize);
    println!("|| game will require {} bytes of dyn mem", static_start);

    println!("|| ... resetting alloc counters prior to constructing interpreter");
    ALLOCATOR.reset_counts();

    let opts = Options { rand_seed: 0x1337 };
    let mut zvm = Zmachine::new(&DATA, DumbUi::default(), opts);

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

    0
}

pub struct DumbUi {
    buf: [u8; 64],
}

impl Default for DumbUi {
    fn default() -> Self {
        DumbUi { buf: [0; 64] }
    }
}

#[link(name = "c")]
extern "C" {
    pub static mut stdin: *mut libc::FILE;
}

impl DumbUi {
    fn fill_input_buf(&mut self) {
        unsafe { libc::fgets(self.buf.as_mut_ptr() as *mut i8, 64, stdin) };
    }
}

impl Ui for DumbUi {
    fn print(&self, text: &str) {
        print!("{}", text);
    }

    fn print_object(&mut self, object: &str) {
        self.print(object);
    }

    fn set_status_bar(&self, left: &str, right: &str) {
        // let _ = (left, right);
        self.print(&format!("{}  -  {}", left, right));
    }

    fn get_input_buf(&mut self) -> &str {
        &core::str::from_utf8(&self.buf[..self.buf.iter().position(|x| *x == 0).unwrap()])
            .expect("cannot parse invalid utf8")
    }
}
