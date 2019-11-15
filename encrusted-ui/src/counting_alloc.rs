use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

/// Modified from the example at
/// https://doc.rust-lang.org/stable/std/alloc/struct.System.html
pub struct CountingAllocator {
    allocated: AtomicUsize,
    high_watermark: AtomicUsize,
}

impl CountingAllocator {
    pub const fn new() -> CountingAllocator {
        CountingAllocator {
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

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            self.allocated.fetch_add(layout.size(), SeqCst);
            let a = self.allocated.load(SeqCst);
            if a > self.high_watermark.load(SeqCst) {
                self.high_watermark.store(a, SeqCst)
            }
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        self.allocated.fetch_sub(layout.size(), SeqCst);
    }
}
