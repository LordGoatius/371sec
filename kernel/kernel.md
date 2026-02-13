---
title: Kernels
author: Jimmy Ostler
options:
  implicit_slide_ends: true
theme:
  name: dark
---

Intro
===
<!-- font_size: 2 -->

Q: So like, what is a kernel? Or like, an operating system? 
<!-- pause -->

A (mine, not the "correct one"):
<!-- incremental_lists: true -->
A kernel (usually):
- Manages Memory [I lied about memory in previous presentations I'm sorry :( ]
- Manages Processes
- Manages access to "operating system resources" (files, threads, streams of data)
- Manages access to ***devices***

Which of these are we doing in our homework?

Devices
---
<!-- font_size: 2 -->

That's right, we were writing to a VGA device! [Devices are how the CPU interacts with external hardware]

Notably, our device was memory mapped.

This is where I lied about memory. Sorry!
<!-- pause -->

***Memory is not a flat layout***

In fact, memory is closer to a list of (start, range) points.
And not all of these are useable the way we think of memory.

<!-- pause -->
Take VGA for instance. It is a point `(0xb8000, 4000)` We can read and write to it, like normal memory, but it isn't
stored as RAM. It's a separate "device", managed by firmware, which responds to our reads and writes.

This is called Memory Mapped Input/Output, or MMIO.

Devices 2
---
<!-- font_size: 2 -->
VGA memory is about the simplest device that can exist.
<!-- pause -->
It acts how we would usually think of memory acting, we read/write, it just happens to have side effects.
<!-- pause -->
If you read carefully, you'll notice each `u16` making up one character called a "register".
<!-- pause -->
MMIO devices are made up of "registers" that we *might* be able to read/write.
More complicated devices have more complicated rules, such as
- Register that cannot be written to, only read.
- Register that cannot be read from, only written.
- Registers can only be written to or read in exact sizes (usually the case).

Example
---

<!-- font_size: 2 -->

A UART (Universal Asynchronous Reciever/Transmitter) Device using MMIO

```rust
pub struct Uart(&'static mut Registers);

struct Registers {
    thr_rbr_dll: Register<ReadWrite, u8>,
    ier_dlh: Register<ReadWrite, u8>,
    iir_fcr: Register<ReadWrite, u8>,
    lcr: Register<ReadWrite, u8>,
    mcr: Register<ReadWrite, u8>,
    lsr: Register<Read, u8>,
    msr: Register<Read, u8>,
    sr: Register<ReadWrite, u8>,
}
```
<!-- pause -->
We could perhaps pretend this was simply a `u64`. But if we do that, we do not
guarantee compatability with the spec, which is where you'll find register descriptions.

Devices, Concl.
---
<!-- font_size: 2 -->
In the VGA case it doesn't seem to matter. But it may in other cases, and this is important to remember.
<!-- pause -->
Device and memory ranges are read from the firmware using something called "DeviceTree" or "ACPI" usually.
This is not *always* the case, but we read from these to find our memory layout [list of (addr, size) points].

We can then use the ranges specified for devices to load device drivers (your homework) and the ranges
specified for system memory as memory.
<!-- pause -->
This leads into the second job of a kernel.

Memory & Processes
---
<!-- font_size: 2 -->
Clearly, memory is not as simple as we had once hoped, but it is clearly more powerful.

It is our job to write an OS that uses system memory efficently and correctly.

I won't go over the details of that *yet*, but the operating system manages

- Who can access memory
- What addresses write where
- Which memory can be executed
- Which memory can be read/written
- Which processes can see what shared memory
- Running programs by loading them into memory
- Giving processes memory when they ask

A lot of this relates to processes, which we'll get into later. Out OS's don't need to do nearly all of this,
because it's still very hard.

OS Resources
---
<!-- font_size: 2 -->
Operating systems also provide their own resources for us to use. We use these through things called "syscalls".

Q: Why can we not run a Windows exe on Linux?
<!-- pause -->
A: [ignoring other reasons] Because system calls and the resources accessed using them are different!

In linux, we might write to a file using a system call of "write", but the same system call on windows could be something like
`_findnext64i32`, or some other random windows system call.
<!-- pause -->
Wine, the linux program for running windows files translates these system calls to the corresponding linux calls at runtime.

If we are running a binary for a different architecture, there needs to be another layer. This is what QEMU is doing, like for the RISC-V homework.

RISC-V Homework
---
<!-- font_size: 2 -->
I'm now going to go over my simplified solution to the RISC-V homework. In order to do this I needed to
write my own (copy from another repository and modify slightly) linker file.

Not only does it run, but it also does hello world.

Linker Script
---
<!-- font_size: 2 -->
Resource for learning about linker script: <span style="color: yellow;">https://mcyoung.xyz/2021/06/01/linker-script/</span>
<!-- font_size: 1 -->
```ld
ENTRY(boot)

SECTIONS {
    . = 0x80000000;

    .text :{
        KEEP(*(.text.boot)); // Keep `boot` function at the beginning
        *(.text .text.*);
        . = ALIGN(8);
        *(.text.stvec)
    }

    .rodata : ALIGN(8) {
        *(.rodata .rodata.*);
    }

    .data : ALIGN(8) {
        *(.data .data.*);
    }

    .bss : ALIGN(8) {
        __bss = .;
        *(.bss .bss.* .sbss .sbss.*);
        __bss_end = .;
    }

    . = ALIGN(16);
    . += 1024 * 1024 * 2;
    __stack_top = .;

    /DISCARD/ : {
        *(.eh_frame);
    }
}
```

Linker Script Cont
---
<!-- font_size: 2 -->
This script tells your compiler how to combine the multiple compiled source files together.
Different part of our program get stored at different places. For example, the code will appear in the
`.text` section. I specify that `.text.boot` should be at the beginning of this, or address
0x80000000, the beginning of where the CPU will start executing.

I additionally have to add something to my code as well.

Linker Script Code
---
```rust
#[unsafe(link_section = ".text.boot")]
#[unsafe(no_mangle)]
pub extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            "la sp, __stack_top",
            "j {main}",
            main = sym main,
            options(noreturn)
        )
    }
}
```

<!-- font_size: 2 -->
I can specify where I want a function to be linked.

This means that the code for this function will start at
0x80000000, and will thus be the first thing executed by QEMU.

The function itself sets the stack up, and then jumps to our main function using assembly.

RISC-V: Hello World
---
<!-- font_size: 2 -->
While the details aren't important, there exists a UART device at address 0x1000_0000.


```rust
const UART: *mut u8 = 0x1000_0000 as *mut u8;

pub fn main() {
    for i in "hello world\r\n".bytes() {
        unsafe {
            UART.write_volatile(i);
        }
    }

    loop {}
}
```

By running this in QEMU, we can get a fully functional hello world (run example).

RISC-V: Additional Files
---
<!-- font_size: 2 -->
I also modified this crate to be runnable using `cargo run`.

I added a `rust-toolchain.toml`:

```toml
[toolchain]
channel = "stable"
targets = ["riscv64imac-unknown-none-elf"]
```

And a runner in the `.cargo/config.toml`

```toml
[target.riscv64imac-unknown-none-elf]
runner = "qemu-system-riscv64 -bios none -machine virt -serial stdio -kernel "
rustflags = ["-Clink-arg=-Tkernel.ld", "-Clinker=rust-lld"]

[build]
target = "riscv64imac-unknown-none-elf"
```

RISC-V Concl.
---
<!-- font_size: 2 -->
These files make it possible to not need any command line args when running the program.
