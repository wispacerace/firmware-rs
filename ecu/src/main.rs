#![no_std]  // don't bring in anything in Rust's standard library
#![no_main] // tell the compiler to not mark or look for a "main function".
            // we'll handle that on our own.

// -- pick a panicking behavior
extern crate panic_halt; // halts on any panics; can breakpoint on `rust_begin_unwind` to catch.

// -- import our device from theß peripheral access crate (or PAC; effectively,
//    a thin layer over register accesses. this is normally most of what a "HAL" gives you).
//	  note! "crate" is Rust parlance for what other languages call a package.
use stm32f4::stm32f413;

// -- import "app" attribute (attributes are sorta like decorators in other languages and sit inside `#[]`)
use rtfm::app;

// -- APP is basically a module that contains some functions. The unit type () is an utter lie.
//    this would _ideally_ be written as an actual module (`mod`); however, using attributes on modules requires a feature gate,
//    which requires a nightly toolchain. To make RTFM work on stable we use the const item instead.
//    **tl;dr**: ignore everything about this beyond the #[app()] attribute and the stuff inside the {} brackets.
#[app(device = stm32f4::stm32f413)] // configures a new RTFM app, setting the device to our peripheral access module (stm32f413)
const APP: () = {
	/// First part of the application to run. `init` runs _with interrupts disabled_ and has _exclusive access_ to
	/// Cortex-M and device-specific peripherals through the passed `c: init::Context` argument (`c.core` and `c.device`, respectively).
	#[init]
	fn init(c: init::Context) {
	}
};