#![no_std]  // don't bring in anything in Rust's standard library
#![no_main] // tell the compiler to not mark or look for a "main function".
            // we'll handle that on our own.

// -- pick a panicking behavior
extern crate panic_halt; // halts on any panics; can breakpoint on `rust_begin_unwind` to catch.

use cortex_m::asm;
use cortex_m_rt::entry; // this imports the entry macro, which gives us that fancy
                        // #[entry] thing that marks the entry point of our program
                        // (which cortex_m_rt manages calling for us). this is why we
                        // turned off implicit main earlier with #![no_main] – cortex_m_rt
                        // handles that all for us.

#[entry]
fn main() -> ! {
    asm::nop(); // prevent main from optimizing to abort in release mode.
                // remove when there is actually code here.

    loop { // test
    }
}
