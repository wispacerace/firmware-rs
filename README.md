# firmware-rs
[abandoned rust prototype] of the rocket firmware

## Why was this abandoned?
The overhead of onboarding people to Rust and RTFM was deemed too high by the embedded systems lead;
they personally preferred it but the impedance mismatches between our existing design and RTFM, as well
as the dearth of peripheral drivers for stm32f4, just wasn't conducive to building something in a constrained
time frame, especially when trying to find people to help!

We're instead going with C++, ChibiOS/RT, ChibiOS/HAL (which provides very nice DMA and blocking I/O).
clang-tidy and the clang static analyzer will be used to provide some additional safety.
We'll be trying to use C++17 where relevant.
