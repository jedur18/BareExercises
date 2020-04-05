//! bare4.rs
//!
//! Access to Peripherals
//!
//! What it covers:
//! - Raw pointers
//! - Volatile read/write
//! - Busses and clocking
//! - GPIO (a primitive abstraction)

#![no_std]
#![no_main]

extern crate panic_halt;

extern crate cortex_m;
use cortex_m_rt::entry;



// Peripheral addresses as constants
#[rustfmt::skip]
mod address {
    pub const PERIPH_BASE: u32      = 0x40000000;
    pub const AHB1PERIPH_BASE: u32  = PERIPH_BASE + 0x00020000;
    pub const RCC_BASE: u32         = AHB1PERIPH_BASE + 0x3800;
    pub const RCC_AHB1ENR: u32      = RCC_BASE + 0x30;
    pub const GBPIA_BASE: u32       = AHB1PERIPH_BASE; //+ 0x0000;
    pub const GPIOA_MODER: u32      = GBPIA_BASE; //+ 0x00;
    pub const GPIOA_BSRR: u32       = GBPIA_BASE + 0x18;
}


use address::*;

// see the Reference Manual RM0368 (www.st.com/resource/en/reference_manual/dm00096844.pdf)
// rcc,     chapter 6
// gpio,    chapter 8

#[inline(always)]
fn read_u32(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const _) }
  //  core::ptr::read_volatile(addr as *const _)
}

#[inline(always)]
fn write_u32(addr: u32, val: u32) {
    unsafe {
        core::ptr::write_volatile(addr as *mut _, val);
    }
}

fn wait(i: u32) {
    for _ in 0..i {
        cortex_m::asm::nop(); // no operation (cannot be optimized out)
    }
}

#[entry]
fn main() -> ! {
    // power on GPIOA
    let r = read_u32(RCC_AHB1ENR); // read
    write_u32(RCC_AHB1ENR, r | 1); // set enable
    
    // configure PA5 as output
    let r = read_u32(GPIOA_MODER) & !(0b11 << (5 * 2)); // read and mask
    write_u32(GPIOA_MODER, r | 0b01 << (5 * 2)); // set output mode
   
    // and alter the data output through the BSRR register
    // this is more efficient as the read register is not needed.

    loop {
        // set PA5 high
        write_u32(GPIOA_BSRR, 1 << 5); // set bit, output hight (turn on led)
    //    wait(10_000);

        // set PA5 low
    //    write_u32(GPIOA_BSRR, 1 << (5 + 16)); // clear bit, output low (turn off led)
    //    wait(10_000);
    }
}

// 0.  Build and run the application (debug build).
//
//    > cargo run --example bare4
//    (or use the vscode)
//
// 1.  Did you enjoy the blinking?
//
/*
        Imposible not enjoying that ;)
*/
//    Now lookup the data-sheets, and read each section referred,
//    6.3.11, 8.4.1, 8.4.7
//
//    Document each low level access *code* by the appropriate section in the
//    data sheet.
/*
    // power on GPIOA
    let r = read_u32(RCC_AHB1ENR); // read
    write_u32(RCC_AHB1ENR, r | 1); // set enable
    // Section 6.3.9 of documentation--> RCC_AHB1ENR enable registers
    // What this does is to make an OR operation of each bit of the register (which has been)
    // saved in r with a 1.
    // In that way you achieve to put every bit at 1 and enable all registers

    // configure PA5 as output
    let r = read_u32(GPIOA_MODER) & !(0b11 << (5 * 2)); // read and mask
    write_u32(GPIOA_MODER, r | 0b01 << (5 * 2)); // set output mode
    // Section 8.4.1 of documentation --> GPIO mode register
    // GPIO ports have 32 bits which allows to configure 16 i/o pins (need 2 bits for configure each pin)
    // In this case we want to set pin 5 of A port as output. Methodology:
    // (1) Define a mask: !(0b11 << (5 * 2))  -> This has 32 (when you make the & with a 32 bit register) bits with value 1 except the two ones
    // located in position 10 and 11 (bits of the register for configuring PA5) that have the value 0
    // (2) Read the register using the mask. In that way, making AND operation you keep the values
    // that have in every bit except the two of PA5 that are set in 0.
    // (3) Define mask for output mode in PA5: 0b01 << (5 * 2) --> This sets a 01 in bits number
    // 10 and 11 respectively, which is the output mode configuration for a pin
    // (4) Write in registers using the mask: It does the OR operation, between r which was
    // obtained in (2) and the mask in (3). In that way every bit keeps as its original value
    // except the two for configuring PA5 which are set to 01 to make PA an output pin.

    // Setting PA5 to 1 (High)
    // set PA5 high
        write_u32(GPIOA_BSRR, 1 << 5); // set bit, output hight (turn on led)
        wait(10_000);

    //Section 8.4.7: GPIOA_BSRR : GPIO port bit set/reset register
    // For this registers that have 32 bits, from 0-15 are used to set the pins to 1
    // and 16-32 to reset the pins. If we want to put PA5 to 1, which we need to do is
    // to write a 1 in the bit number 5 of the register (If we want to reset that pin
    // we would need to write a 1 in the bit number 21 of the register).
    // What the write instruction does is to set the bit number 5 to 1 and hence PA5 is 
    // high now
    
    //Setting PA5 to 0 (LOW)
    // set PA5 low
        write_u32(GPIOA_BSRR, 1 << (5 + 16)); // clear bit, output low (turn off led)
        wait(10_000);
    
    // Section 8.4.7: GPIOA_BSRR : GPIO port bit set/reset register
    // As mentioned in previous part to reset a pin (put it to LOW), what we need to do
    // is to write a 1 in the bit from 16 to 32 of the pin that we want to modify. In this case
    // as its the pin 5, we put a 1 in the bit number 5+16 = 21

    */
//    Commit your answers (bare4_1)
//
// 2. Comment out line 40 and uncomment line 41 (essentially omitting the `unsafe`)
//
//    //unsafe { core::ptr::read_volatile(addr as *const _) }
//    core::ptr::read_volatile(addr as *const _)
//
//    What was the error message and explain why.
//
/*
        The error message says that calling to an unsafe function is unsafe and it requires an
        unsafe function or block.
        It is because read_volatile can behave unsafetely (See safety part in https://doc.rust-lang.org/std/ptr/fn.read_volatile.html)
        so it need to be inside the unsafe function
*/
//
//    Digging a bit deeper, why do you think `read_volatile` is declared `unsafe`.
//    (https://doc.rust-lang.org/core/ptr/fn.read_volatile.html, for some food for thought )
//
/*
        pub unsafe fn read_volatile<T>(src: *const T) -> T
        It is declared unsafe because there are cases when it can lead up to undefined behaviour
        That occurs when the src is not valid to read or is not properly aligned.
        Since read_volatile create a bitwise copy of T regardless(idependientemente) T is copy or not
        In case T is not copy using both the returned value and the value at *src, can violate
        memory safety.
*/
//    Commit your answers (bare4_2)
//
// 3. Volatile read/writes are explicit *volatile operations* in Rust, while in C they
//    are declared at type level (i.e., access to varibles declared volatile amounts to
//    volatile reads/and writes).
//
//    Both C and Rust (even more) allows code optimization to re-order operations, as long
//    as data dependencies are preserved.
//
//    Why is it important that ordering of volatile operations are ensured by the compiler?
//
/*
    It is important because because it is crucial that some operations are executed before others.
    If not, writing and reading could end in unexpected behaviours and results.
*/
//    Give an example in the above code, where reordering might make things go horribly wrong
//    (hint, accessing a peripheral not being powered...)
//
/*
    For example in previous code you need to power the GPIOA before its configuration. Another example could be
    for example that you need to set up a pin as output before using it as an output (Taking in account that the
    pin is not in output mode by default)
*/
//    Without the non-reordering property of `write_volatile/read_volatile` could that happen in theory
//    (argue from the point of data dependencies).
/*
   As some of the operations are dependant of other executed before, it should not happen with volatile operations.
   Without this properties things can go horribly wrong and end in not desired results.
*/
//    Commit your answers (bare4_3)
