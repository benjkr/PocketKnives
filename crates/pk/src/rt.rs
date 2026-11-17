//! The few symbols a `no_std` exe linked with `/NODEFAULTLIB` needs that the C runtime would
//! otherwise supply. Not built for `cargo test`, which links the real runtime.
use core::arch::asm;
use core::ffi::c_void;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    // panic = "abort" in every profile: nothing unwinds. A panic is a bug; end at once.
    crate::exit(0xC000_0409) // STATUS_STACK_BUFFER_OVERRUN's code, what __fastfail reports
}

/// The compiler references `_fltused` from any code that uses floating point (an MSVC convention).
#[unsafe(no_mangle)]
pub static _fltused: i32 = 0;

/// The prebuilt `core`/`alloc` carry MSVC unwind tables naming this personality routine. With
/// panic = "abort" nothing ever unwinds, so it is never called.
#[unsafe(no_mangle)]
pub extern "C" fn __CxxFrameHandler3() -> ! {
    crate::exit(0xC000_0409)
}

// The mem functions LLVM calls for copies, fills and compares. Written with string instructions:
// a plain loop here would be recognised as memcpy and compiled into a call to itself. Fast on
// every x64 CPU since Ivy Bridge (ERMSB).

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    unsafe { asm!("rep movsb", inout("rcx") n => _, inout("rdi") dst => _, inout("rsi") src => _, options(nostack, preserves_flags)) };
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let (d, s) = (dst as *mut u8, src as *const u8);
    if (d as usize).wrapping_sub(s as usize) >= n {
        // No overlap that a forward copy would clobber.
        unsafe { asm!("rep movsb", inout("rcx") n => _, inout("rdi") d => _, inout("rsi") s => _, options(nostack, preserves_flags)) };
    } else {
        // dst starts inside src: copy backwards, then restore the direction flag the ABI expects.
        unsafe {
            asm!("std", "rep movsb", "cld", inout("rcx") n => _, inout("rdi") d.add(n - 1) => _, inout("rsi") s.add(n - 1) => _, options(nostack))
        };
    }
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    unsafe { asm!("rep stosb", inout("rcx") n => _, inout("rdi") dst => _, in("al") c as u8, options(nostack, preserves_flags)) };
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32 {
    let (a, b) = (a as *const u8, b as *const u8);
    let mut i = 0;
    while i < n {
        let (x, y) = unsafe { (*a.add(i), *b.add(i)) };
        if x != y {
            return x as i32 - y as i32;
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(a: *const c_void, b: *const c_void, n: usize) -> i32 {
    unsafe { memcmp(a, b, n) }
}

/// LLVM turns a scan for a zero `u16` into a `wcslen` call; `repne scasw` can't be turned back.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wcslen(s: *const u16) -> usize {
    let mut left: usize = usize::MAX;
    unsafe { asm!("repne scasw", inout("rcx") left, inout("rdi") s => _, in("ax") 0u16, options(nostack, readonly)) };
    !left - 1
}

/// The stack probe MSVC-target code calls before a frame larger than a page (also stable
/// `slice::sort`'s scratch buffer): touches each 4 KB page below `rsp` in order, so the guard page
/// grows the stack instead of being skipped. `rax` holds the size; only flags change, and the
/// caller moves `rsp` itself (the same contract as the C runtime's and compiler-rt's).
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __chkstk() {
    core::arch::naked_asm!(
        "push rcx",
        "push rax",
        "cmp rax, 0x1000",
        "lea rcx, [rsp + 24]",
        "jb 3f",
        "2:",
        "sub rcx, 0x1000",
        "test qword ptr [rcx], rcx",
        "sub rax, 0x1000",
        "cmp rax, 0x1000",
        "ja 2b",
        "3:",
        "sub rcx, rax",
        "test qword ptr [rcx], rcx",
        "pop rax",
        "pop rcx",
        "ret",
    )
}
