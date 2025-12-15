<!-- @format -->

# Blog Post 2

Need to set the version to Nightly:

```rust
rustup override set nightly
```

verify version:

```rust
rustc --version
```

BIOS sits on ROM

On x86, there are two firmware standards:

1. the “Basic Input/Output System“ (BIOS) and
2. the newer “Unified Extensible Firmware Interface” (UEFI)

## Bootloader

- <https://github.com/rust-osdev/bootimage>
- automatically prepends a bootloader to your kernel.
- Open Bootloader Standard - Multiboot <https://wiki.osdev.org/Multiboot>

### GNU GRUB

- Grand Unified Bootloader
- Most popular bootloader for Linux systems.

##

```rust
rustup component add rust-src
```

Blog OS Code:

```rust
cargo build --target x86_64-blog_os.json
```

ours:

```rust
cargo build --target x86_64-bare_metal.json
```

```rust
cargo build --target arm-bare_metal.json
```

## Print To Screen

use VGA Text mode

Cargo Crate:
bootloader = "0.11.13"
"An experimental x86_64 bootloader that works on both BIOS and UEFI systems."

The bootimage tool performs the following steps behind the scenes:

1. It compiles our kernel to an ELF file.
2. It compiles the bootloader dependency as a standalone executable.
3. It links the bytes of the kernel ELF file to the bootloader.

```qemu
 qemu-system-x86_64 -drive format=raw,file=target/x86_64-bare_metal/debug/bootimage-bare_metal.bin
```

## Final Code

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
```
