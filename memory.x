MEMORY
{
    /* NOTE THAT:
       1 K = 1 KiBi = 1024 bytes

       These memory regions have been adjusted to match the
       STM32F413 memory layout */

    /* TODO: FIX THESE FOR STM32F413 */
    FLASH : ORIGIN = 0x00000000, LENGTH = 256K
    RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* the CALL STACK goes here.
   it is a full-descending stack, ie, it starts at _stack_start
   and grows downwards. this is actually the default value,
   but we write it out explicitly for ~learning~! */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

