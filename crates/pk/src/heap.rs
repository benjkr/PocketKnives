//! The global allocator: the process heap (`HeapAlloc`), as the C runtime's `malloc` uses.
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use windows_sys::Win32::System::Memory::{GetProcessHeap, HEAP_ZERO_MEMORY, HeapAlloc, HeapFree, HeapReAlloc};

/// The heap aligns every block to 16 bytes on x64 (`MEMORY_ALLOCATION_ALIGNMENT`).
const HEAP_ALIGN: usize = 16;

pub struct ProcessHeap;

#[cfg(not(test))]
#[global_allocator]
static HEAP: ProcessHeap = ProcessHeap;

impl ProcessHeap {
    /// Over-aligned blocks: allocate extra, align inside, keep the real pointer just before.
    unsafe fn alloc_aligned(&self, l: Layout, flags: u32) -> *mut u8 {
        let raw = unsafe { HeapAlloc(GetProcessHeap(), flags, l.size() + l.align()) } as *mut u8;
        if raw.is_null() {
            return raw;
        }
        let aligned = ((raw as usize + l.align()) & !(l.align() - 1)) as *mut u8;
        unsafe { (aligned as *mut *mut u8).sub(1).write(raw) };
        aligned
    }
}

unsafe impl GlobalAlloc for ProcessHeap {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if l.align() <= HEAP_ALIGN {
            unsafe { HeapAlloc(GetProcessHeap(), 0, l.size()) as *mut u8 }
        } else {
            unsafe { self.alloc_aligned(l, 0) }
        }
    }

    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        if l.align() <= HEAP_ALIGN {
            unsafe { HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, l.size()) as *mut u8 }
        } else {
            unsafe { self.alloc_aligned(l, HEAP_ZERO_MEMORY) }
        }
    }

    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        let block = if l.align() <= HEAP_ALIGN { p } else { unsafe { (p as *mut *mut u8).sub(1).read() } };
        unsafe { HeapFree(GetProcessHeap(), 0, block as *const c_void) };
    }

    unsafe fn realloc(&self, p: *mut u8, l: Layout, new_size: usize) -> *mut u8 {
        if l.align() <= HEAP_ALIGN {
            return unsafe { HeapReAlloc(GetProcessHeap(), 0, p as *const c_void, new_size) as *mut u8 };
        }
        let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, l.align()) };
        let q = unsafe { self.alloc(new_layout) };
        if !q.is_null() {
            unsafe { core::ptr::copy_nonoverlapping(p, q, l.size().min(new_size)) };
            unsafe { self.dealloc(p, l) };
        }
        q
    }
}
