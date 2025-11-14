// This is free and unencumbered software released into the public domain.
// Author: Griffin Evans <griffinevans@protonmail.com>
#![no_std]
#![no_main]

mod advice;

use core::sync::atomic::{self, Ordering};
use advice::ADVICE;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

unsafe fn syscall(n: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret
}

// getrandom(buf, count, flags) - syscall 318
unsafe fn getrandom(buf: &mut [u8]) -> usize {
    unsafe {
        syscall(318, buf.as_mut_ptr() as usize, buf.len(), 0)
    }
}

// write(fd, buf, count) - syscall 1
unsafe fn write(fd: usize, buf: &[u8]) -> usize {
    unsafe {
        syscall(1, fd, buf.as_ptr() as usize, buf.len())
    }
}

// sys_exit(error_code) - syscall 610
unsafe fn exit(error_code: usize) -> ! {
    unsafe {
        syscall(60, error_code, 0, 0);
        loop {}
    }
}

// memset implementation needed by compiler for array initialization
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        let mut i = 0;
        while i < n {
            *s.add(i) = c as u8;
            i += 1;
        }
        s
    }
}

// rust_eh_personality implementation needed by compiler for panic handling
// This function is used when rust unwinds a stack during a panic, but since
// we build with panic = "abort", this is never used. Hence, it can be sefely
// left as an empty method
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        let mut buf = [0u8; 8];
        getrandom(&mut buf);

        let idx = usize::from_ne_bytes(buf) % ADVICE.len();
        write(1, ADVICE[idx].as_bytes());
        write(1, b"\n");

        exit(0);
    }
}
