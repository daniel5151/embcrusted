use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering::SeqCst};

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {}
}

pub struct MallocFreeAlloc {
    allocated: AtomicUsize,
    high_watermark: AtomicUsize,
}

impl MallocFreeAlloc {
    pub const fn new() -> MallocFreeAlloc {
        MallocFreeAlloc {
            allocated: AtomicUsize::new(0),
            high_watermark: AtomicUsize::new(0),
        }
    }

    pub fn reset_counts(&self) {
        self.allocated.store(0, SeqCst);
        self.high_watermark.store(0, SeqCst);
    }

    pub fn get_current_usage(&self) -> usize {
        self.allocated.load(SeqCst)
    }

    pub fn get_high_watermark(&self) -> usize {
        self.high_watermark.load(SeqCst)
    }
}

unsafe impl GlobalAlloc for MallocFreeAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocated.fetch_add(layout.size(), SeqCst);
        let a = self.allocated.load(SeqCst);
        if a > self.high_watermark.load(SeqCst) {
            self.high_watermark.store(a, SeqCst)
        }

        libc::malloc(layout.size() as libc::size_t) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.allocated.fetch_sub(layout.size(), SeqCst);
        libc::free(ptr as *mut libc::c_void)
    }
}
