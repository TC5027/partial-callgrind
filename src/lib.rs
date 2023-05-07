//! Easy to use client requests to start/stop callgrind at specific location of your code for precise profiling (100% Rust).
//!
//! For now only x86_64 is supported.
//!
//! # Examples
//!
//! Skip `Vec` initialization code and profile only sort.
//! Compile in release mode and then use `valgrind --tool=callgrind --collect-atstart=no --instr-atstart=no {exec}`.
//!
//! ```
//! use partial_callgrind::{start, stop};
//! use rand::Rng;
//!
//! fn main() {
//!     let mut rng = rand::thread_rng();
//!
//!     let mut data: Vec<u8> = (0..10_000).into_iter().map(|_| rng.gen::<u8>()).collect();
//!     start();
//!     data.sort();
//!     stop();
//! }
//! ```
use std::arch::asm;

const MAGIC_NUMBER: u64 = 1129578496;

#[doc(hidden)]
#[inline(always)]
unsafe fn request(default: u64, args: &[u64; 6]) -> u64 {
    let result;
    asm!(
    "rol rdi, 3
        rol rdi, 13
        rol rdi, 61
        rol rdi, 51
        xchg rbx,rbx",
    inout("rdx") default=>result,
    in("rax") args.as_ptr()
    );
    result
}

#[inline(always)]
pub fn toggle_collection() {
    unsafe { request(0, &[MAGIC_NUMBER + 2, 0, 0, 0, 0, 0]) };
}

#[inline(always)]
pub fn start_instrumentation() {
    unsafe { request(0, &[MAGIC_NUMBER + 4, 0, 0, 0, 0, 0]) };
}

#[inline(always)]
pub fn stop_instrumentation() {
    unsafe { request(0, &[MAGIC_NUMBER + 5, 0, 0, 0, 0, 0]) };
}

/// Start instrumentation and toggle collection state.
/// With `--collect-atstart=no` option, callgrind's collection state is disabled at the beginning.
#[inline(always)]
pub fn start() {
    start_instrumentation();
    toggle_collection();
}

/// Toggle collection state and stop instrumentation.
#[inline(always)]
pub fn stop() {
    toggle_collection();
    stop_instrumentation();
}
