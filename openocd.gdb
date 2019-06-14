target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit (don't want infinite trace loops)
set backtrace limit 32

# break on unhandled exceptions, hardfaults, and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

# try to pause execution at the user entry point
break main

# turn on semihosting
monitor arm semihosting enable

# load the binary to the device
load

# start the process and immediately halt
stepi