#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use libc_alloc::LibcAlloc;
use libc_print::std_name::*;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[alloc_error_handler]
unsafe fn alloc_error(layout: core::alloc::Layout) -> ! {
    eprintln!("memory allocation of {} bytes failed", layout.size());
    libc::abort();
}

#[panic_handler]
unsafe fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    eprintln!("a panic occured");
    libc::abort();
}

////////////////////////////////////////////////////////////////////////////////

use alloc::string::String;
use alloc::vec::Vec;

use encrusted::{Options, Zmachine, UI};

fn open_story_file(argc: isize, argv: *const *const u8) -> Vec<u8> {
    if argc != 2 {
        panic!("must pass valid story file as only argument");
    }

    let mut data: Vec<u8> = Vec::new();
    unsafe {
        let fname = *argv.offset(1);

        let file = libc::fopen(fname as _, b"rb\0".as_ptr() as _);
        let mut buf = [0; 4096];

        if file.is_null() {
            panic!("could not open story file");
        }

        while libc::fread(buf.as_mut_ptr() as _, 1, 4096, file) == 4096 {
            data.extend(buf.iter());
        }
        data.extend(buf.iter());

        libc::fclose(file);
    }

    data
}

#[no_mangle]
pub extern "C" fn main(argc: isize, argv: *const *const u8) -> isize {
    let data = open_story_file(argc, argv);

    let ui = NostdTermUi;

    let mut opts = Options::default();
    opts.rand_seed = 0;

    let mut zvm = Zmachine::new(data, ui, opts).expect("could not construct z-machine");

    zvm.run();

    0
}

pub struct NostdTermUi;

impl UI for NostdTermUi {
    fn clear(&self) {}

    fn print(&mut self, text: &str) {
        print!("{}", text);
    }

    fn debug(&mut self, text: &str) {
        print!("{}", text);
    }

    fn print_object(&mut self, object: &str) {
        print!("{}", object);
    }

    fn set_status_bar(&self, _left: &str, _right: &str) {}

    fn get_user_input(&self) -> String {
        extern "C" {
            pub static mut stdin: *mut libc::FILE;
        }

        let mut buf = [0; 64];
        unsafe { libc::fgets(buf.as_mut_ptr() as _, 64, stdin) };
        String::from_utf8((&buf[..buf.iter().position(|x| *x == 0).unwrap()]).to_vec())
            .expect("user entered invalid utf8")
    }

    fn reset(&self) {
        println!();
    }
}
