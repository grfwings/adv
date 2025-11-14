// This is free and unencumbered software released into the public domain.
// Author: Griffin Evans <griffinevans@protonmail.com>
#![no_std]
#![no_main]

use core::sync::atomic::{self, Ordering};

const ADVICE: &[&str] = &[
    "Write clearly - don't be too clever.",
    "Say what you mean, simply and directly.",
    "Use library functions.",
    "Avoid temporary variables.",
    "Write clearly - don't sacrifice clarity for \"efficiency.\"",
    "Let the machine do the dirty work.",
    "Replace repetitive expressions by calls to a common function.",
    "Parenthesize to avoid ambiguity.",
    "Choose variable names that won't be confused.",
    "Avoid unnecessary branches.",
    "Use the good features of a language; avoid the bad ones.",
    "Use the \"telephone test\" for readability.",
    "Make your programs read from top to bottom.",
    "Use arrays to avoid repetitive control sequences.",
    "Don't stop with your first draft.",
    "Modularize. Use subroutines.",
    "Each module should do one thing well.",
    "Make sure every module hides something.",
    "Don't patch bad code - rewrite it.",
    "Write and test a big program in small pieces.",
    "Test input for validity and plausibility.",
    "Make sure input cannot violate the limits of the program.",
    "Identify bad input; recover if possible.",
    "Make input easy to prepare and output self-explanatory.",
    "Localize input and output in subroutines.",
    "Make sure all variables are initialized before use.",
    "[During debugging…] Don't stop at one bug.",
    "Watch out for off-by-one errors.",
    "Avoid multiple exits from loops.",
    "Test programs at their boundary values.",
    "Program defensively.",
    "10.0 times 0.1 is hardly ever 1.0.",
    "Don't compare floating point numbers just for equality.",
    "Make it right before you make it faster.",
    "Keep it right when you make it faster.",
    "Make it clear before you make it faster.",
    "Don't sacrifice clarity for small gains in \"efficiency.\"",
    "Keep it simple to make it faster.",
    "Don't diddle code to make it faster - find a better algorithm.",
    "Make sure comments and code agree.",
    "Don't just echo the code with comments - make every comment count.",
    "Don't comment bad code - rewrite it.",
    "Use variable names that mean something.",
    "Indent to show the logical structure of a program.",
    "Debugging is twice as hard as writing the code in the first place. \nTherefore, if you write the code as cleverly as possible, you are, \nby definition, not smart enough to debug it.",
    "Consider how you would solve your immediate problem without adding anything new.",
    "Any organization that designs a system (defined broadly) will produce a design whose structure is a copy of the organization's communication structure.",
    "A complex system that works has evolved from a simple system that worked.\nA complex system built from scratch won't work.",
    "With a sufficient number of users of an API, it does not matter what you promise in the contract: all observable behaviors of your system will be depended on by somebody.",
    "Premature optimization is the root of all evil.",
    "Given enough eyeballs, all bugs are shallow.",
    "Be conservative in what you send, liberal in what you accept.",
];

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
