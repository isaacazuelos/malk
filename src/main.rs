//! Experiments with bare metal rust programming on an Pro Micro.

// This feature lets us use inline assembly, used to avoid optimizing away our
// busy loop.
#![feature(llvm_asm)]

// We don't have an operating system or allocator, so `std` is not an option.
#![no_std]

// We need to extern our main differently so that the linker can find it. See
// the "blog_os" resource link in the README for more.
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// These constants can be found in the ATmega32u4 datasheet.
const DDR_B: *mut u8 = 0x05 as _;
const PORT_B: *mut u8 = 0x24 as _;

#[no_mangle]
pub extern fn main() {
    // First we have to configure our pin as an output. The RX_LED is Port B,
    // Pin 0 according to the pin out diagram referenced in the readme.

    // We set it as output by setting the "Data Direction" bit for the pin. This
    // means setting the DD_xn_ bit in the DDR_x_ register, where _x_ is the
    // port's letter, and _n_ is the pin's number in it's port. Setting it to 1
    // means output, 0 is output.

    // We're going to just set the whole port as output.
    unsafe { *DDR_B = 0xFF };

    loop {
        // Now that our pin is an output, we can to set it to high or low to
        // control the LED state. This is done by writing a 1 to PORT_xn_ for
        // high or on, and 0 for low or off. We're just going to set the whole
        // port again.
        unsafe { *PORT_B = 0xFF };

        delay();

        unsafe { *PORT_B = 0x00 };

        // You need two delays. This too _way_ too long to catch, but otherwise
        // the loop restarts and immediately turns it back on before the LED can
        // noticeably turn off.
        delay();
    }
}

/// A small busy loop, copied from the avr-rust/blink repo mentioned in the
/// readme.
fn delay() {
    for _ in 0..400000 {
        // This prevents the loop from being optimized away. An astute reader
        // will notice there's no actual assembly here. All "volatile" means is
        // "don't optimize this" so we're left with an intentional non-optimized
        // do nothing.
        //
        // Finding what this is exactly was a bit of a journey. The
        // [Unstable][1] book very helpfully says:
        //
        // > volatile - specifying this is analogous to `__asm__ __volatile__
        // > (...)` in gcc/clang
        //
        // And the [GCC online docs][2] say:
        //
        // > GCC’s optimizers sometimes discard asm statements if they determine
        // > there is no need for the output variables. Also, the optimizers may
        // > move code out of loops if they believe that the code will always
        // > return the same result (i.e. none of its input values change
        // > between calls). Using the volatile qualifier disables these
        // > optimizations
        //
        // [1]: https://doc.rust-lang.org/beta/unstable-book/library-features/llvm-asm.html#options
        // [2]: https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html#Volatile
        unsafe { llvm_asm!("" :::: "volatile")}
    }
}